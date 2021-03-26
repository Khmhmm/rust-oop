use crate::{BoxedArg, BoxedReturn};

pub struct Method<C: ?Sized> {
    inner: Box<dyn Fn(&C, BoxedArg) -> BoxedReturn>
}
impl<C> Method<C>{
    #[inline]
    pub fn call(&self, c: &C, arg: BoxedArg) -> BoxedReturn {
        (self.inner)(c, arg)
    }
}

impl<C> From<Box<dyn Fn(&C, BoxedArg) -> BoxedReturn>> for Method<C> {
    fn from( f: Box<dyn Fn(&C, BoxedArg) -> BoxedReturn> ) -> Self {
        Method {
            inner: f
        }
    }
}

impl<C> std::ops::Deref for Method<C> {
    type Target = Box<dyn Fn(&C, BoxedArg) -> BoxedReturn>;
    fn deref(&self) -> &Box<dyn Fn(&C, BoxedArg) -> BoxedReturn> {
        &self.inner
    }
}

// impl<C> ClassMember<C> for Method<C> {
//     fn take(&self, caller: &C, arg: BoxedArg) -> BoxedReturn {
//         self.call(caller, arg)
//     }
// }

pub type BoxedRawMethod<C> = Box<dyn Fn(&C, BoxedArg) -> BoxedReturn>;


pub struct MethodMut<C: ?Sized> {
    inner: Box<dyn FnMut(&mut C, BoxedArg) -> BoxedReturn>
}
impl<C> MethodMut<C>{
    #[inline]
    pub fn call(&mut self, c: &mut C, arg: BoxedArg) -> BoxedReturn {
        (self.inner)(c, arg)
    }
}


impl<C> From<Box<dyn FnMut(&mut C, BoxedArg) -> BoxedReturn>> for MethodMut<C> {
    fn from( f: Box<dyn FnMut(&mut C, BoxedArg) -> BoxedReturn> ) -> Self {
        MethodMut {
            inner: f
        }
    }
}

pub type BoxedRawMethodMut<C> = Box<dyn FnMut(&mut C, BoxedArg) -> BoxedReturn>;
