use std::fmt::Debug;

pub trait MethodArg: Debug {}

impl<T: Debug> MethodArg for T{}
pub type BoxedArg = Box<dyn MethodArg>;

pub trait MethodReturnable: Debug{}
impl<T: Debug> MethodReturnable for T{}

pub type BoxedReturn = Box<dyn MethodReturnable>;
