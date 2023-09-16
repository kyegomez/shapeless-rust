use std::any::Any;
use std::fmt::Debug;

pub trait Shapeless: Debug + Any {
    fn as_any(&self) -> &dyn Any;
    fn type_name(&self) -> &'static str;
}

impl<T: Shapeless> Shapeless for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

pub fn wrapper<T: Shapeless>(arg: T) {
    println!("{:?}", arg);
}

pub fn is_type<T: Shapeless>(arg: &dyn Shapeless) -> bool {
    arg.as_any().is::<T>()
}

pub fn downcast_ref<T: Shapeless>(arg: &dyn Shapeless) -> Option<&T> {
    arg.as_any().downcast_ref::<T>()
}

pub fn convert<T: Shapeless, U: Shapeless>(arg: &T) -> Result<&U, &'static str> {
    if let Some(val) = arg.as_any().downcast_ref::<U>() {
        Ok(val)
    } else {
        Err("Conversion failed")
    }
}