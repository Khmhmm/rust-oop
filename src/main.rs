use std::collections::HashMap;
use std::ops::Deref;

/// Macro, allowing you to call parents' methods with any number of arguments, or without it.
/// Please note, that you HAVE TO implement `.base(mut self) -> (YourParent, MetaObject)`
/// and `.backstep(YourParent, MetaObject) -> Self` methods, macro will not work without them!
/// See ExampleParent and ExampleChild implementations for more info.
/// Also don't forget about macro autobase! and autobackstep!
#[macro_export]
macro_rules! base {
    ($child: ident, $child_type: ty, $base_type: ty, $action: ident) => {
        let (mut base, keep) = $child.base();
        let base_ptr = &mut base;
        <$base_type>::$action(base_ptr);
        let $child = <$child_type>::backstep(base, keep);
    };

    // ($child: ident, $child_type: ty, $base_type: ty, $action: ident, $arg: expr) => {
    //     let (mut base, keep) = $child.base();
    //     <$base_type>::$action(&base, $arg);
    //     let $child = <$child_type>::backstep(base, keep);
    // }

    ($child: ident, $child_type: ty, $base_type: ty, $action: ident, $($args: expr),*) => {
        let (mut base, keep) = $child.base();
        let base_ptr = &mut base;
        <$base_type>::$action(base_ptr, $($args),*);
        let $child = <$child_type>::backstep(base, keep);
    }
}

/// Allows you to implement base method automatically. All you need is to type inside the signature:
/// `autobase!(self, ParentType)`
#[macro_export]
macro_rules! autobase {
    ($this: ident, $parent: ty) => {
        let meta = $this.untyped();
        return (<$parent>::cast(meta.clone()),meta.clone());
    }
}

/// Allows you to implement backstep method automatically. All you need to type inside the signature:
/// `autobackstep!(parent_object_arg, metaobject_arg, Self)`
#[macro_export]
macro_rules! autobackstep {
    ($base: ident, $keep: ident, $child: ty) => {
        let meta = MetaObject::merge($base.untyped(), $keep);
        return <$child>::cast(meta);
    }
}

/// Type representing class's fields. Note that values are all in string format, because it is easy
/// to parse anything from a string. You may use serde_json or any other serializing/deserializing instrument
/// to handle 'cast' function
#[derive(Clone)]
pub struct MetaObject(HashMap<String,String>);

impl From<MetaObject> for HashMap<String, String> {
    fn from(obj: MetaObject) -> Self {
        obj.0
    }
}

impl From<HashMap<String, String>> for MetaObject {
    fn from(obj: HashMap<String, String>) -> Self {
        Self(obj)
    }
}

impl MetaObject {
    #[inline]
    pub fn as_map(&self) -> &HashMap<String, String> {
        &self.0
    }

    pub fn merge(updated: MetaObject, keep: MetaObject) -> MetaObject {
        let mut updated: HashMap<String, String> = updated.into();
        let mut keep: HashMap<String, String> = keep.into();
        for (k,v) in updated.drain() {
            let ptr = keep.get_mut(&k).unwrap();
            *ptr = v;
        }

        return keep.into();
    }
}

/// Trait, describing transformation betwen classes
pub trait Class {
    /// Function, transforming an object to a meta-object
    fn cast(map: MetaObject) -> Self;
    /// Function, describing deserialization a class from a meta-object
    fn untyped(&self) -> MetaObject;
}

pub mod inheritance {
    use crate::Class;

    pub struct Base;

    impl Base {
        pub fn base(mut self) -> (Base, crate::MetaObject) {
            panic!("Unable to call base class of the base class!");
        }

        pub fn backstep<T>(any: T, keep: crate::MetaObject) -> Self {
            panic!("Unable to backstep the base class!");
        }
    }

    impl Class for Base {
        fn cast(map: crate::MetaObject) -> Self {
            Self{}
        }

        fn untyped(&self) -> crate::MetaObject {
            std::collections::HashMap::new().into()
        }
    }


    #[inline]
    pub fn cast<F: Class, T: Class>(obj: &F) -> T {
        T::cast(Class::untyped(obj))
    }
}


/// An example class, showing implementation in detail.
/// For inheritance example, check the ExampleChild struct.
struct ExampleParent {
    field: i32
}

impl ExampleParent {
    /// Our parent's class own method
    fn do_something(&self) -> i32 {
        println!("Done something! {}", self.field);
        self.field
    }

    /// Parent's class with 1 argument
    fn set_something(&mut self, n: i32) {
        self.field = n;
    }

    /// Parent's class with 2 arguments
    fn field_sum(&mut self, rho: i32, lho: i32) {
        self.field = rho + lho;
    }

    /// Base method, calling class's parent instance
    fn base(mut self) -> (crate::inheritance::Base, MetaObject) {
        return (crate::inheritance::Base{},self.untyped())
    }

    /// Method, constructing object back from base object and saved state
    fn backstep(base: crate::inheritance::Base, keep: MetaObject) -> Self {
        let meta = MetaObject::merge(base.untyped(), keep);
        Self::cast(meta)
    }
}

impl Class for ExampleParent {
    #[inline]
    fn cast(map: MetaObject) -> Self {
        let map: HashMap<String, String> = map.into();
        Self {
            field: map["field"].parse().unwrap()
        }
    }

    fn untyped(&self) -> MetaObject {
        let mut hm = HashMap::new();
        hm.insert("field".to_string(), format!("{}", self.field));
        return hm.into();
    }
}

/// An example class, representing a child's implementation
/// ```
/// let child = ExampleChild::new(1,"hi".to_string());
/// base!(child, ExampleChild, ExampleParent, set_something, 15);
/// println!("{}",child.field);
/// base!(child, ExampleChild, ExampleParent, field_sum, 155, 255);
/// println!("{}",child.field);
/// ```
struct ExampleChild {
// Parent's fields
    field: i32,
// Child's fields
    data: String
}

impl ExampleChild {
    /// Child's constructor
    fn new(field: i32, data: String) -> ExampleChild {
        Self{field, data}
    }

    #[inline]
    /// Child's own method
    fn do_else(&self) -> String {
        format!("{}:{}", self.field, self.data)
    }

    #[inline]
    /// Parent's field setter
    fn set_field(&mut self, field: i32) {
        self.field = field;
    }

    #[inline]
    /// Child's base method. Note that it is implemented with macro autobase!
    /// All you need is to type self and parent's type: `autobase!(self, ExampleParent);`
    fn base(self) -> (ExampleParent, MetaObject) {
        // let meta = self.untyped();
        // return (ExampleParent::cast(meta.clone()),meta.clone());
        autobase!(self, ExampleParent);
    }

    #[inline]
    /// Child's backstep method, updating current state from parent's state.
    /// Note that it is implemented with macro autobase!
    /// All you need is to type first arg, second arg and Self!
    fn backstep(base: ExampleParent, keep: MetaObject) -> Self {
        // let meta = MetaObject::merge(base.untyped(), keep);
        // return Self::cast(meta);
        autobackstep!(base, keep, Self);
    }
}

impl Class for ExampleChild {
    #[inline]
    fn cast(map: MetaObject) -> Self {
        let map: HashMap<String, String> = map.into();
        Self {
            field: map["field"].parse().unwrap(),
            data: map["data"].clone()
        }
    }

    fn untyped(&self) -> MetaObject {
        let mut hm = HashMap::new();
        hm.insert("field".to_string(), format!("{}", self.field));
        hm.insert("data".to_string(), self.data.clone());
        return hm.into();
    }
}

fn main() {
}
