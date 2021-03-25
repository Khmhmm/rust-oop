use crate::{BoxedReturn, ClassMember, BoxedArg};

pub struct Field {
    inner: BoxedReturn
}

// impl<T, C> ClassMember<C> for Field<T> {
//     fn take(&self, _caller: &C, _: BoxedArg) -> BoxedReturn {
//         self.inner
//     }
// }
