use crate::*;
use std::ffi::{CStr, CString};
use regex::*;
use lazy_static::*;

const MAX_PROGRAM_INFO_LOG_SIZE: usize = 1024;
const VERSION_NUMBER: &'static str = "#version 450";
pub const FEATURE_VIEW_UNIFORM_NAME: &'static str = "_u_view";
pub const FEATURE_PROJECTION_UNIFORM_NAME: &'static str = "_u_projection";

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum ShaderFeature {
    View,
    Projection,
}

impl ShaderFeature {
    fn from_name(name: impl AsRef<str>) -> Self {
        let name = name.as_ref();
        match name {
            "view" => ShaderFeature::View,
            "projection" => ShaderFeature::Projection,
            _ => panic!("Unknown shader feature {:?}", name),
        }
    }

    fn inserted_code(self) -> String {
        match self {
            ShaderFeature::View => format!("
uniform mat4 {0};
mat4 applyView(mat4 a) {{
    return {0} * a;
}}

vec4 applyView(vec4 a) {{
    return {0} * a;
}}", FEATURE_VIEW_UNIFORM_NAME),

            ShaderFeature::Projection => format!("
uniform mat4 {0};
mat4 applyProjection(mat4 a) {{
    return {0} * a;
}}

vec4 applyProjection(vec4 a) {{
    return {0} * a;
}}", FEATURE_PROJECTION_UNIFORM_NAME),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum ShaderDirective {
    Features(Vec<String>),
}

impl ShaderDirective {
    fn from_name(name: impl AsRef<str>) -> Self {
        let name = name.as_ref().trim();

        match name {
            _ => panic!("Unknown shader compiler directive {:?}", name),
        }
    }

    fn from_name_args(name: impl AsRef<str>, mut args: Vec<impl Into<String>>,) -> Self {
        let name = name.as_ref().trim();
        let args = args.drain(..).map(|a| a.into()).collect::<Vec<String>>();

        match name {
            "feature" => ShaderDirective::Features(args),
            _ => panic!("Unknown shader compiler directive {:?}", name),
        }
    }

    fn inserted_code(&self) -> Option<String> {
        match self {
            ShaderDirective::Features(features) => {
                let mut inserted = String::new();
                for feature in features {
                    inserted.push_str(&ShaderFeature::from_name(feature).inserted_code());
                    inserted.push_str("\n");
                }
                Some(inserted)
            },
            _ => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum ShaderStage {
    Vertex,
    Fragment,
}

impl ShaderStage {
    pub fn gl_enum(&self) -> GLenum {
        match self {
            ShaderStage::Vertex => gl::VERTEX_SHADER,
            ShaderStage::Fragment => gl::FRAGMENT_SHADER,
        }
    }

    pub fn stage_bit(&self) -> GLenum {
        match self {
            ShaderStage::Vertex => gl::VERTEX_SHADER_BIT,
            ShaderStage::Fragment => gl::FRAGMENT_SHADER_BIT,
        }
    }

    fn apply_postprocess(&self, code: &mut String) -> Vec<ShaderFeature> {
        // Compiled regexes
        lazy_static! {
            static ref RE_DIRECTIVE: Regex = Regex::new(r"#\[(.*?)\]").unwrap();
            static ref RE_NAME_ARGS: Regex = Regex::new(r"#\[(.*?)\((.*?)\)").unwrap();
        }

        // Remove carriage returns and version number
        *code = code.replace("\r", "").replace(VERSION_NUMBER, "");

        // Search for directives
        let mut directives = RE_DIRECTIVE.find_iter(&code).map(|mat|{
            // Isolate just the directive string
            let directive = &code[mat.range()];

            // Check the formatting of the directive
            if let Some(name_args) = RE_NAME_ARGS.captures(directive) {
                // DIRECTIVENAME(ARGS)
                // Get the name
                let name = &directive[name_args.get(1).unwrap().range()];

                // Get the args in the parentheses, separated by commas
                let args = directive[name_args.get(2).unwrap().range()].split(',').map(|arg| arg.trim()).collect();

                // Create a ShaderDirective object for this directive
                ShaderDirective::from_name_args(name, args)
            }
            else {
                // DIRECTIVENAME or other
                // Create a ShaderDirective object for this directive
                ShaderDirective::from_name(directive)
            }
        }).collect::<Vec<ShaderDirective>>();

        // Remove directives from code
        *code = RE_DIRECTIVE.replace(&code, "").into_owned();

        // Work on directives
        // First do inserts
        let mut final_insert = String::new();
        for inserted in directives.iter().map(ShaderDirective::inserted_code) {
            if let Some(inserted) = inserted {
                final_insert.insert_str(0, &inserted);
            }
        }
        final_insert.insert_str(0, "\n");
        code.insert_str(0, &final_insert);

        // Insert version number
        code.insert_str(0, &format!("{}\n", VERSION_NUMBER));

        // Return enabled features
        #[allow(irrefutable_let_patterns)]
        directives.drain(..).filter_map(|directive| if let ShaderDirective::Features(features) = directive { Some(features) } else { None }).flatten().map(|name| ShaderFeature::from_name(name)).collect()
    }
}

pub struct Program {
    gl_handle: IntHandle,
    stage: ShaderStage,
    features: Vec<ShaderFeature>,
}

impl Program {
    pub fn new(stage: ShaderStage, code: impl Into<String>) -> Self {
        // Convert code to an owned string
        let mut code = code.into();

        // Apply post-processing
        let features = stage.apply_postprocess(&mut code);

        // Convert code to a C-string
        let code = CString::new(code).unwrap();

        // Store pointer to code in an array so we can create a pointer to the pointer safely
        let code_ptrs = [code.as_ptr() as *const _];

        // Create the shader program object
        let gl_handle = unsafe { gl::CreateShaderProgramv(stage.gl_enum(), 1, code_ptrs.as_ptr()) };

        // Get the info log for the program
        let mut length: GLsizei = 0;
        let mut info_log: [GLchar; MAX_PROGRAM_INFO_LOG_SIZE] = [0; MAX_PROGRAM_INFO_LOG_SIZE];
        unsafe {
            gl::GetProgramInfoLog(
                gl_handle,
                MAX_PROGRAM_INFO_LOG_SIZE as GLsizei,
                &mut length as *mut _,
                info_log.as_mut_ptr(),
            )
        };

        // Print the info log if it's not empty
        if length > 0 {
            let message_slice = unsafe {
                std::slice::from_raw_parts(info_log.as_ptr() as *const u8, length as usize)
            };
            let message_vec = message_slice.to_owned();
            let message = CString::new(message_vec).unwrap();
            println!("\x1B[35m{}\nShader code:\n{}\n\x1B[37m", message.to_str().unwrap(), generate_numbered_code(&code));
        }

        Self { gl_handle, stage, features }
    }

    pub fn stage(&self) -> ShaderStage {
        self.stage
    }

    pub fn uniform_location(&self, name: impl AsRef<str>) -> Option<GLuint> {
        let name = CString::new(name.as_ref()).unwrap();
        let location = unsafe { gl::GetUniformLocation(self.handle(), name.as_ref().as_ptr()) };
        if location < 0 {
            None
        } else {
            Some(location as GLuint)
        }
    }

    pub fn set_uniform_mat4f(&self, location: GLuint, mats: &[Mat4f]) {
        unsafe {
            gl::ProgramUniformMatrix4fv(
                self.handle(),
                location as GLint,
                mats.len() as i32,
                gl::FALSE,
                mats.as_ptr() as *const _,
            )
        };
    }

    pub fn shader_features(&self) -> &[ShaderFeature] {
        &self.features
    }
}

impl GLHandle for Program {
    fn handle(&self) -> IntHandle {
        self.gl_handle
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        if self.gl_handle != 0 {
            unsafe { gl::DeleteProgram(self.gl_handle) };
        }
    }
}

fn generate_numbered_code(code: &CStr) -> String {
    let code = code.to_str().unwrap().split('\n');
    let mut new_code = String::new();
    for (line_number, line) in code.enumerate() {
        new_code.push_str(&format!("{:03} {}\n", line_number + 1, line));
    }
    new_code
}