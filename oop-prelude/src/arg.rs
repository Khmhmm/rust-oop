pub trait MethodArg {}

impl<T> MethodArg for T{}
pub type BoxedArg = Box<dyn MethodArg>;

pub trait MethodReturnable{}
impl<'a, T> MethodReturnable for &'a T{}

pub type BoxedReturn = Box<dyn MethodReturnable>;
