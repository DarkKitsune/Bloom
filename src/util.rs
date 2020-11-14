pub use gl::types::GLchar;
pub use gl::types::GLenum;
pub use gl::types::GLint;
pub use gl::types::GLsizei;
pub use gl::types::GLsizeiptr;
pub use gl::types::GLuint;
pub type Mat4f = fennec_algebra::Matrix<f32, 4, 4>;
pub type Vec2f = fennec_algebra::Vector<f32, 2>;
pub type Vec3f = fennec_algebra::Vector<f32, 3>;
pub type Vec4f = fennec_algebra::Vector<f32, 4>;
pub type Vec2d = fennec_algebra::Vector<f64, 2>;
pub type Vec3d = fennec_algebra::Vector<f64, 3>;
pub type Vec4d = fennec_algebra::Vector<f64, 4>;
pub type Vec2u = fennec_algebra::Vector<u32, 2>;
pub type Vec3u = fennec_algebra::Vector<u32, 3>;
pub type Vec4u = fennec_algebra::Vector<u32, 4>;
pub type Vec2i = fennec_algebra::Vector<i32, 2>;
pub type Vec3i = fennec_algebra::Vector<i32, 3>;
pub type Vec4i = fennec_algebra::Vector<i32, 4>;

pub const DEBUG: bool = cfg!(debug_assertions);

#[macro_export]
macro_rules! init_array {
    ([$value_type:ty; $count:expr], mut $value_fn:expr) => {{
        use std::mem::MaybeUninit;
        let mut func = $value_fn;
        let mut array: MaybeUninit<[$value_type; $count]> = MaybeUninit::uninit();
        unsafe {
            for idx in 0..$count {
                *(array.as_mut_ptr() as *mut $value_type).add(idx) = func(idx);
            }
            array.assume_init()
        }
    }};
    ([$value_type:ty; $count:expr], $value_fn:expr) => {{
        use std::mem::MaybeUninit;
        let func = $value_fn;
        let mut array: MaybeUninit<[$value_type; $count]> = MaybeUninit::uninit();
        unsafe {
            for idx in 0..$count {
                *(array.as_mut_ptr() as *mut $value_type).add(idx) = func(idx);
            }
            array.assume_init()
        }
    }};
}

pub trait CollectArray<I> {
    fn collect_or_default<const COUNT: usize>(self) -> [I; COUNT]
    where
        I: Default + Copy;

    fn collect_array<const COUNT: usize>(self) -> [I; COUNT];
}

impl<I, T: Iterator<Item = I>> CollectArray<I> for T {
    fn collect_or_default<const COUNT: usize>(mut self) -> [I; COUNT]
    where
        I: Default + Copy,
    {
        let mut array = [Default::default(); COUNT];
        for elem in array.iter_mut().take(COUNT) {
            if let Some(item) = self.next() {
                *elem = item;
            } else {
                break;
            }
        }
        array
    }

    fn collect_array<const COUNT: usize>(mut self) -> [I; COUNT] {
        init_array!([I; COUNT], mut |_| if let Some(item) = self.next() {
            item
        } else {
            panic!("Iterator did not provide enough items to fill the array!");
        })
    }
}
