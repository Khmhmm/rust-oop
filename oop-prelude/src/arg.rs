pub trait MethodArg {}

impl<T> MethodArg for T{}
pub type BoxedArg = Box<dyn MethodArg>;

pub trait MethodReturnable{}
impl<T> MethodReturnable for T{}

pub type BoxedReturn = Box<dyn MethodReturnable>;
