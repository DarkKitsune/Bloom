use fennec_algebra::Vector;

pub trait DrawTarget {
    fn size() -> Vector<u32, 2>;
}
