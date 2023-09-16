use std::any::Any;
use std::fmt::Debug;

pub trait Shapeless: Debug + Any {
    fn as_any(&self) -> &dyn Any;
}

pub fn wrapper<T: Shapeless>(arg: T) {
    println("{:?}", arg);
}