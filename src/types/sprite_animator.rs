use crate::*;
use std::cell::RefCell;
use std::mem::swap;
use std::rc::Rc;

const COMPUTE_SHADER: &str = "
#[feature(batch, deltaTime)]
layout (local_size_x = 128) in;

struct Actor
{
    vec4 positionVelocity;
    vec4 scaleVelocity;
    vec4 rotationUpdatedAnimtime;
    vec4 rectangle;
};

struct Instance
{
    mat4 transform;
    vec4 rectangle;
};

layout (std140, binding = 0) buffer buffer_Actors
{
    Actor actors[];
};

layout (std140, binding = 1) buffer buffer_Instances
{
    Instance instances[];
};

mat4 mat_scale(vec3 scale)
{
    return mat4(
            scale.x,    0,          0,          0,
            0,          scale.y,    0,          0,
            0,          0,          scale.z,    0,
            0,          0,          0,          1
        );
}

mat4 mat_translate(vec3 trans)
{
    return mat4(
            1,          0,          0,          0,
            0,          1,          0,          0,
            0,          0,          1,          0,
            trans.x,    trans.y,    trans.z,    1
        );
}

mat4 mat_rotateZ(float angle)
{
    float cos = cos(angle);
    float sin = sin(angle);
    return mat4(
            cos,        sin,        0,          0,
            -sin,       cos,        0,          0,
            0,          0,          1,          0,
            0,          0,          0,          1
        );
}

mat4 buildMatrix(Actor actor)
{
    mat4 mat = mat_translate(vec3(actor.positionVelocity.xy, 0.0)) * mat_scale(vec3(actor.scaleVelocity.xy, 1.0)) * mat_rotateZ(actor.rotationUpdatedAnimtime.x);
    return mat;
}

Instance buildInstance(Actor actor)
{
    Instance vinst;
    vinst.transform = buildMatrix(actor);
    vinst.rectangle = actor.rectangle;
    return vinst;
}

void main() 
{
    uint index = batchOffset() + gl_GlobalInvocationID.x;
    Actor actor = actors[index];
    actor.positionVelocity.xy += actor.positionVelocity.zw * deltaTime();
    instances[index] = buildInstance(actor);
    actors[index] = actor;
}";

const MAX_DISPATCH: GLuint = 128;

#[derive(Clone, Debug)]
pub struct SpriteAnimator {
    compute_pipeline: Rc<RefCell<ComputePipeline>>,
    actor_buffer: Option<Rc<RefCell<Buffer>>>,
    instance_buffer: Option<Rc<RefCell<Buffer>>>,
}

impl SpriteAnimator {
    pub fn new() -> Self {
        let program = Program::new(ShaderStage::Compute, COMPUTE_SHADER);
        let compute_pipeline = Rc::new(RefCell::new(ComputePipeline::new(program)));

        Self {
            compute_pipeline,
            actor_buffer: None,
            instance_buffer: None,
        }
    }

    pub fn set_buffers(
        &mut self,
        actor_buffer: Rc<RefCell<Buffer>>,
        instance_buffer: Rc<RefCell<Buffer>>,
    ) {
        self.actor_buffer = Some(actor_buffer);
        self.instance_buffer = Some(instance_buffer);
    }

    pub fn animate(&mut self, gfx: &mut GFX, delta_time: f64) {
        let mut actor_buffer = None;
        let mut instance_buffer = None;
        swap(&mut actor_buffer, &mut self.actor_buffer);
        swap(&mut instance_buffer, &mut self.instance_buffer);
        if DEBUG && (actor_buffer.is_none() || instance_buffer.is_none()) {
            panic!("SpriteAnimator buffers are not set");
        }
        if let Some(delta_uniform_location) = self.compute_pipeline.borrow().program().uniform_location(FEATURE_DELTA_TIME_UNIFORM_NAME) {
            self.compute_pipeline.borrow().program().set_uniform_f(delta_uniform_location, delta_time as f32);
        }
        gfx.dispatch_compute_1d(
            self.compute_pipeline.borrow_mut(),
            &[
                actor_buffer.clone().unwrap().borrow(),
                instance_buffer.clone().unwrap().borrow(),
            ],
            0..actor_buffer.clone().unwrap().borrow().length() as GLuint,
            MAX_DISPATCH,
        );
        swap(&mut actor_buffer, &mut self.actor_buffer);
        swap(&mut instance_buffer, &mut self.instance_buffer);
    }

    pub fn actor_buffer(&self) -> Option<&Rc<RefCell<Buffer>>> {
        self.actor_buffer.as_ref()
    }

    pub fn instance_buffer(&self) -> Option<&Rc<RefCell<Buffer>>> {
        self.instance_buffer.as_ref()
    }
}
