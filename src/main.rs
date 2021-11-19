use std::collections::HashMap;

pub trait Class {
    fn cast(map: HashMap<String,String>) -> Self where Self: Sized;
    fn untyped(&self) -> HashMap<String, String>;
}

pub mod inheritance {
    use crate::Class;

    pub struct Base;

    impl Class for Base {
        fn cast(map: std::collections::HashMap<String, String>) -> Self {
            Self{}
        }

        fn untyped(&self) -> std::collections::HashMap<String, String> {
            std::collections::HashMap::new()
        }
    }


    #[inline]
    pub fn cast<F: Class, T: Class>(obj: &F) -> T {
        T::cast(Class::untyped(obj))
    }
}



struct Example {
    field: i32
}

impl Example {
    fn do_something(&self) -> i32 {
        println!("Done something! {}", self.field);
        self.field
    }

    fn base(&self) -> crate::inheritance::Base {
        crate::inheritance::Base{}
    }
}

impl Class for Example {
    #[inline]
    fn cast(map: HashMap<String, String>) -> Self {
        Self {
            field: map["field"].parse().unwrap()
        }
    }

    fn untyped(&self) -> HashMap<String, String> {
        let mut hm = HashMap::new();
        hm.insert("field".to_string(), format!("{}", self.field));
        return hm;
    }
}

struct ExampleChild {
// Parent's fields
    field: i32,
// Child's fields
    data: String
}

impl ExampleChild {
    fn new(field: i32, data: String) -> ExampleChild {
        Self{field, data}
    }

    #[inline]
    fn do_else(&self) -> String {
        format!("{}:{}", self.field, self.data)
    }

    #[inline]
    fn set_field(&mut self, field: i32) {
        self.field = field;
    }

    #[inline]
    fn base(&self) -> Example {
        inheritance::cast::<ExampleChild, Example>(self)
    }
}

impl Class for ExampleChild {
    #[inline]
    fn cast(map: HashMap<String, String>) -> Self {
        Self {
            field: map["field"].parse().unwrap(),
            data: map["data"].clone()
        }
    }

    fn untyped(&self) -> HashMap<String, String> {
        let mut hm = HashMap::new();
        hm.insert("field".to_string(), format!("{}", self.field));
        hm.insert("data".to_string(), self.data.clone());
        return hm;
    }
}




fn main() {
    let mut child = ExampleChild::new(1,"hi".to_string());
    let parent = inheritance::cast::<ExampleChild, Example>(&child);
    parent.do_something();
    child.set_field(2);
    let base = child.base();
    base.do_something();
}
