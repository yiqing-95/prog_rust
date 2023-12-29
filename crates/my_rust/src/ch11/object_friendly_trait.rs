
// 这个不是dyn trait 友好的定义 带Self 返回了
pub trait Spliceable {
    fn splice(&self, other: &Self) -> Self;
}


// This trait is compatible with trait objects

pub trait MegaSpliceable {
    fn splice(&self, other: &dyn MegaSpliceable) -> Box<dyn MegaSpliceable>;
}