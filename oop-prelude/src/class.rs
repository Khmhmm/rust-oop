use crate::{Method, MethodArg, BoxedArg, BoxedReturn};

pub trait ClassCalls{
    fn call(&self, name: &str, arg: BoxedArg) -> BoxedReturn where Self: Sized;
    fn take_method(&self, name: &str) -> Option< &Method<Self> > where Self: Sized;
}

pub trait ClassConstructor{
    fn construct() -> Self;
}

pub trait ClassDefinition {
    fn get_methods() -> Vec<(String, Method<Self>)> where Self: Sized;
}
