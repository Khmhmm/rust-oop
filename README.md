# rust-oop
Object-oriented programming implementation in Rust. Please, do not use it in your project! This realization is just a research.

# About
Please, DO NOT use this in your projects!
This repo is just a research project, where I'm trying to inspect if it is possible to implement inheritance in Rust (when incapsulation and polymorphism with traits are already here).

# Usage
At first, create Parent class and implement 'Class' trait.

```
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
```

Then, create child class and make base points on Parent.
```
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
```

Okay, now we can create child's instance and call methods of the parent.

```
let child = ExampleChild::new(1,"hi".to_string());
base!(child, ExampleChild, ExampleParent, set_something, 15);
println!("{}",child.field);
base!(child, ExampleChild, ExampleParent, field_sum, 155, 255);
println!("{}",child.field);
```

Great. But still you should not use this in production.

# License

MIT License
-----------

Copyright (c) 2021 Matvey Zaharov
Permission is hereby granted, free of charge, to any person
obtaining a copy of this software and associated documentation
files (the "Software"), to deal in the Software without
restriction, including without limitation the rights to use,
copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the
Software is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice shall be
included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES
OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT
HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
OTHER DEALINGS IN THE SOFTWARE.
