use crate::BoxedArg;

pub trait Inherit<T>{
    fn inherent_call(caller: &T, arg: BoxedArg);
}
