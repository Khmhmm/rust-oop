pub trait MethodArg {}

impl<T> MethodArg for T{}
pub type BoxedArg = Box<dyn MethodArg>;

pub trait MethodReturnable: std::fmt::Debug{}
impl<T: std::fmt::Debug> MethodReturnable for T{}

pub type BoxedReturn = Box<dyn MethodReturnable>;
