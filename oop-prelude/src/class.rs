use crate::{Method, MethodArg, BoxedArg, BoxedReturn};

pub trait ClassCalls{
    fn call(&self, name: &str, arg: BoxedArg) -> BoxedReturn where Self: Sized;
}

pub trait ClassConstructor{
    fn construct() -> Self;
}

pub trait ClassDefinition {
    fn get_methods() -> Vec<(String, Box<Method<Self>>)> where Self: Sized;
}

pub trait ClassMember<C>{
    fn take(&self, caller: &C, arg: BoxedArg) -> BoxedReturn;
}

pub type BoxedMember<C> = Box<dyn ClassMember<C>>;
