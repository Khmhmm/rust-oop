use std::collections::HashMap;
use std::ops::Deref;
use oop_prelude::*;


struct Parent {
    methods: HashMap<String, Method<Parent>>
}

impl ClassDefinition for Parent {
    fn get_methods() -> Vec<(String, Method<Parent>)> {
        let mut v = Vec::<(String, Method<Parent>)>::new();
        v.push(
            ("show_name".to_string(),
                {
                    let f: BoxedRawMethod<Parent> =
                        to_box!(|p, _| { println!("hello world"); to_box!( () ) })
                    ;
                    f.into()
                }
            )
        );

        v
    }
}

impl ClassConstructor for Parent {

    fn construct() -> Parent {
        Parent{
            methods: Self::get_methods().into_iter().collect()
        }
    }

}

impl ClassCalls for Parent {
    fn call(&self, name: &str, arg: BoxedArg) -> BoxedReturn {
        let method = self.methods.get(name).expect(&format!("No such method: {}", name));
        method.call(&self, arg)
    }

    fn take_method(&self, name: &str) -> Option<&Method<Parent>> {
        self.methods.get(name)
    }
}

impl std::ops::Index<&str> for Parent
{
    type Output = Method<Self>;

    fn index(&self, i: &str) -> &Method<Self> {
        self.take_method(i).expect(&format!("No such method: {}", i))
    }
}

fn main() {
    let p = Parent::construct();
    call!(&p, "show_name", Box::new(5i32));
}
