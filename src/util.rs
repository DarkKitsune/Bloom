pub use gl::types::GLchar;
pub use gl::types::GLenum;
pub use gl::types::GLfloat;
pub use gl::types::GLint;
pub use gl::types::GLintptr;
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
pub type Vec2l = fennec_algebra::Vector<i64, 2>;
pub type Vec3l = fennec_algebra::Vector<i64, 3>;
pub type Vec4l = fennec_algebra::Vector<i64, 4>;
pub use fennec_algebra::Quaternion;
pub use fennec_algebra::Vector;
pub use paste::*;

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

pub trait IntTools {
    fn is_power_of_2(self) -> bool;
}

impl IntTools for i8 {
    fn is_power_of_2(self) -> bool {
        if self == 0 {
            false
        } else {
            (self & (self - 1)) == 0
        }
    }
}

impl IntTools for u8 {
    fn is_power_of_2(self) -> bool {
        if self == 0 {
            false
        } else {
            (self & (self - 1)) == 0
        }
    }
}

impl IntTools for i16 {
    fn is_power_of_2(self) -> bool {
        if self == 0 {
            false
        } else {
            (self & (self - 1)) == 0
        }
    }
}

impl IntTools for u16 {
    fn is_power_of_2(self) -> bool {
        if self == 0 {
            false
        } else {
            (self & (self - 1)) == 0
        }
    }
}

impl IntTools for i32 {
    fn is_power_of_2(self) -> bool {
        if self == 0 {
            false
        } else {
            (self & (self - 1)) == 0
        }
    }
}

impl IntTools for u32 {
    fn is_power_of_2(self) -> bool {
        if self == 0 {
            false
        } else {
            (self & (self - 1)) == 0
        }
    }
}

impl IntTools for i64 {
    fn is_power_of_2(self) -> bool {
        if self == 0 {
            false
        } else {
            (self & (self - 1)) == 0
        }
    }
}

impl IntTools for u64 {
    fn is_power_of_2(self) -> bool {
        if self == 0 {
            false
        } else {
            (self & (self - 1)) == 0
        }
    }
}

pub trait VecfTools {
    type AngleType;
    fn from_angle(radians: Self::AngleType) -> Self;
}

impl VecfTools for Vec2f {
    type AngleType = f32;
    fn from_angle(radians: Self::AngleType) -> Self {
        Self::new([radians.cos(), radians.sin()])
    }
}

pub trait AngleTools {
    type ForwardType;
    fn forward(&self) -> Self::ForwardType;
}

impl AngleTools for f32 {
    type ForwardType = Vec2f;
    fn forward(&self) -> Self::ForwardType {
        Self::ForwardType::from_angle(*self)
    }
}

#[macro_export]
macro_rules! buildable_struct {
    (pub struct $name:ident {$($item_name:ident: $item_type:ty $(= $default:expr)?),*$(,)?}) => {
        pub struct $name {
            $($item_name: $item_type),*
        }

        paste!(
            pub struct [<$name Builder>] {
                $($item_name: Option<$item_type>),*
            }

            impl $name {
                pub fn builder() -> [<$name Builder>] {
                    [<$name Builder>]::new()
                }
            }

            impl [<$name Builder>] {
                pub fn new() -> Self {
                    Self {
                        $($item_name: None),*
                    }
                }

                $(pub fn [<with_ $item_name>] (mut self, $item_name: $item_type) -> Self {
                    self.$item_name = Some($item_name);
                    self
                })*

                pub fn build(self) -> $name {
                    $name {
                        $($item_name: {
                            let mut default: Vec<$item_type> = vec![$($default:expr)?];
                            if default.len() == 0 {
                                self.$item_name.expect(&format!("Field never set: {:?}", stringify!($item_name)))
                            }
                            else {
                                self.$item_name.unwrap_or(default.drain(..).next().unwrap())
                            }
                        }),*
                    }
                }
            }
        );
    };
}

#[macro_export]
macro_rules! path {
    ($($part:expr),*$(,)?) => {
        {
            let mut path = std::path::PathBuf::new();
            $(path.push($part);)*
            path
        }
    };
}
