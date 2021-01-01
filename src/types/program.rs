use crate::*;
use lazy_static::*;
use regex::*;
use std::ffi::{CStr, CString};

const MAX_PROGRAM_INFO_LOG_SIZE: usize = 1024;
const VERSION_NUMBER: &str = "#version 450";
pub const FEATURE_CAMERA_VIEW_UNIFORM_NAME: &str = "_u_view";
pub const FEATURE_CAMERA_PROJECTION_UNIFORM_NAME: &str = "_u_projection";
pub const FEATURE_TRANSFORM_UNIFORM_NAME: &str = "_u_transform";

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum ShaderFeature {
    Camera,
    Transform,
    Noise,
}

impl ShaderFeature {
    fn from_name(name: impl AsRef<str>) -> Self {
        let name = name.as_ref();
        match name {
            "camera" => ShaderFeature::Camera,
            "transform" => ShaderFeature::Transform,
            "noise" => ShaderFeature::Noise,
            _ => panic!("Unknown shader feature {:?}", name),
        }
    }

    fn inserted_code(self) -> String {
        match self {
            ShaderFeature::Camera => format!(
                "
uniform mat4 {0};
mat4 applyView(mat4 a) {{
    return {0} * a;
}}

vec4 applyView(vec4 a) {{
    return {0} * a;
}}

vec3 camForward() {{
    return -vec3({0}[0].z, {0}[1].z, {0}[2].z);
}}

vec3 camRight() {{
    return vec3({0}[0].x, {0}[1].x, {0}[2].x);
}}

vec3 camUp() {{
    return -vec3({0}[0].y, {0}[1].y, {0}[2].y);
}}

vec3 camPosition() {{
    return -vec3({0}[0].w, {0}[1].w, {0}[2].w);
}}

uniform mat4 {1};
mat4 applyProjection(mat4 a) {{
    return {1} * a;
}}

vec4 applyProjection(vec4 a) {{
    return {1} * a;
}}",
                FEATURE_CAMERA_VIEW_UNIFORM_NAME, FEATURE_CAMERA_PROJECTION_UNIFORM_NAME,
            ),
            ShaderFeature::Transform => format!(
                "
uniform mat4 {0};
mat4 applyTransform(mat4 a) {{
    return {0} * a;
}}",
                FEATURE_TRANSFORM_UNIFORM_NAME,
            ),
            ShaderFeature::Noise => String::from("
vec3 randomNormal(vec3 pos) {{
    return normalize(vec3(
        cos(pos.x * 2981.2412512 + sin(pos.y * 239.21585190 + cos(pos.z * 923.9287664) * 976.56895432) * 4574.9856189),
        cos(pos.y * 8145.32161212 + sin(pos.z * 177.1658568 + cos(pos.x * 743.126898) * 7569.142156) * 4123.4584516),
        cos(pos.z * 6354.862316 + sin(pos.x * 445.96213 + cos(pos.y * 512.458127845) * 4123.841261) * 865.622312)
    ));
}}

vec3 cellPos(vec3 pos) {{
    return vec3(floor(pos.x), floor(pos.y), floor(pos.z));
}}

float cellIntensity(vec3 pos, vec3 cellOffset) {{
    vec3 cell = cellPos(pos) + cellOffset;
    vec3 normal = randomNormal(cell);
    float directionIntensity = (dot(pos - cell, normal) + 1.0) / 2.0;
    float distanceIntensity = max(0.0, 1.0 - pow(length(pos - cell), 0.95));
    return directionIntensity * distanceIntensity;
}}

float sigmoid(float x) {{
    return 1.0 / (1.0 + exp(x * -15 + 7.5));
}}

float sampleNoise(vec3 sample_pos, float seed, float scale, int iterations) {{
    float sum = 0.0;
    float divider = 0.0;
    for (int i = 0; i < iterations; i++) {{
        float power = 1.0 / pow(1.4, i);
        sample_pos += seed * 0.5;
        float pos_scale = scale * pow(2.0, i);
        vec3 pos = sample_pos * pos_scale;
        sum +=
            (cellIntensity(pos, vec3(-1.0, -1.0, -1.0)) +
            cellIntensity(pos, vec3(-1.0, -1.0, 0.0)) +
            cellIntensity(pos, vec3(-1.0, -1.0, 1.0)) +
            cellIntensity(pos, vec3(-1.0, 0.0, -1.0)) +
            cellIntensity(pos, vec3(-1.0, 0.0, 0.0)) +
            cellIntensity(pos, vec3(-1.0, 0.0, 1.0)) +
            cellIntensity(pos, vec3(-1.0, 1.0, -1.0)) +
            cellIntensity(pos, vec3(-1.0, 1.0, 0.0)) +
            cellIntensity(pos, vec3(-1.0, 1.0, 1.0)) +
            cellIntensity(pos, vec3(0.0, -1.0, -1.0)) +
            cellIntensity(pos, vec3(0.0, -1.0, 0.0)) +
            cellIntensity(pos, vec3(0.0, -1.0, 1.0)) +
            cellIntensity(pos, vec3(0.0, 0.0, -1.0)) +
            cellIntensity(pos, vec3(0.0, 0.0, 0.0)) +
            cellIntensity(pos, vec3(0.0, 0.0, 1.0)) +
            cellIntensity(pos, vec3(0.0, 1.0, -1.0)) +
            cellIntensity(pos, vec3(0.0, 1.0, 0.0)) +
            cellIntensity(pos, vec3(0.0, 1.0, 1.0)) +
            cellIntensity(pos, vec3(1.0, -1.0, -1.0)) +
            cellIntensity(pos, vec3(1.0, -1.0, 0.0)) +
            cellIntensity(pos, vec3(1.0, -1.0, 1.0)) +
            cellIntensity(pos, vec3(1.0, 0.0, -1.0)) +
            cellIntensity(pos, vec3(1.0, 0.0, 0.0)) +
            cellIntensity(pos, vec3(1.0, 0.0, 1.0)) +
            cellIntensity(pos, vec3(1.0, 1.0, -1.0)) +
            cellIntensity(pos, vec3(1.0, 1.0, 0.0)) +
            cellIntensity(pos, vec3(1.0, 1.0, 1.0))) * power;
        divider += power;
    }}
    return sigmoid(sum / divider);
}}",
            ),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum ShaderDirective {
    Features(Vec<String>),
}

impl ShaderDirective {
    #[allow(clippy::match_single_binding)]
    fn from_name(name: impl AsRef<str>) -> Self {
        let name = name.as_ref().trim();

        match name {
            _ => panic!("Unknown shader compiler directive {:?}", name),
        }
    }

    fn from_name_args(name: impl AsRef<str>, mut args: Vec<impl Into<String>>) -> Self {
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
                    inserted.push('\n');
                }
                Some(inserted)
            } //_ => None,
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
        let mut directives = RE_DIRECTIVE
            .find_iter(&code)
            .map(|mat| {
                // Isolate just the directive string
                let directive = &code[mat.range()];

                // Check the formatting of the directive
                if let Some(name_args) = RE_NAME_ARGS.captures(directive) {
                    // DIRECTIVENAME(ARGS)
                    // Get the name
                    let name = &directive[name_args.get(1).unwrap().range()];

                    // Get the args in the parentheses, separated by commas
                    let args = directive[name_args.get(2).unwrap().range()]
                        .split(',')
                        .map(|arg| arg.trim())
                        .collect();

                    // Create a ShaderDirective object for this directive
                    ShaderDirective::from_name_args(name, args)
                } else {
                    // DIRECTIVENAME or other
                    // Create a ShaderDirective object for this directive
                    ShaderDirective::from_name(directive)
                }
            })
            .collect::<Vec<ShaderDirective>>();

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
        directives
            .drain(..)
            .filter_map(|directive| {
                if let ShaderDirective::Features(features) = directive {
                    Some(features)
                } else {
                    None
                }
            })
            .flatten()
            .map(ShaderFeature::from_name)
            .collect()
    }
}

#[derive(Debug)]
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

        if DEBUG {
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
                println!(
                    "\x1B[35m{}\nShader code:\n{}\n\x1B[37m",
                    message.to_str().unwrap(),
                    generate_numbered_code(&code)
                );
            }
        }

        Self {
            gl_handle,
            stage,
            features,
        }
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

    pub fn set_uniform_texture_unit(&self, location: GLuint, unit: GLuint) {
        unsafe { gl::ProgramUniform1i(self.handle(), location as GLint, unit as GLint) };
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
