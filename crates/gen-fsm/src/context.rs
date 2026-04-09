pub trait Context: Copy + Clone + PartialEq + Eq + core::fmt::Debug {
    const COUNT: usize;
    fn to_index(&self) -> usize;
    fn from_index(index: usize) -> Option<Self>;
}
