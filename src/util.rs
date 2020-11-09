pub use gl::types::GLchar;
pub use gl::types::GLenum;
pub use gl::types::GLint;
pub use gl::types::GLsizei;
pub use gl::types::GLsizeiptr;
pub use gl::types::GLuint;

#[macro_export]
macro_rules! init_array {
    ([$value_type:ty; $count:expr], $value_fn:expr) => {{
        use std::mem::MaybeUninit;
        let mut array: MaybeUninit<[$value_type; $count]> = MaybeUninit::uninit();
        unsafe {
            for idx in 0..$count {
                *(array.as_mut_ptr() as *mut $value_type).offset(idx as isize) = ($value_fn)(idx);
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
        for idx in 0..COUNT {
            if let Some(item) = self.next() {
                array[idx] = item;
            } else {
                break;
            }
        }
        array
    }

    fn collect_array<const COUNT: usize>(mut self) -> [I; COUNT] {
        init_array!([I; COUNT], |idx| if let Some(item) = self.next() {
            item
        } else {
            panic!("Iterator did not provide enough items to fill the array!");
        })
    }
}
