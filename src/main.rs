use std::collections::HashMap;
use std::ops::Deref;
use oop_prelude::*;


struct Parent {
    members: HashMap<String, BoxedMember<Parent>>
}

impl ClassDefinition for Parent {
    fn get_methods() -> Vec<(String, Box<Method<Parent>>)> {
        let mut v = Vec::<(String, Box<Method<Parent>>)>::new();
        v.push(
            ("show_name".to_string(),
                {
                    let f: BoxedRawMethod<Parent> =
                        to_box!(|p, _| { println!("hello world"); to_box!( &() ) })
                    ;
                    Box::new(Method::<Parent>::from(f))
                }
            )
        );

        v
    }
}

impl ClassConstructor for Parent {

    fn construct() -> Parent {
        Parent{
            members: Self::get_methods()
                .into_iter()
                .map(|(s, m)| (s, m as Box<dyn ClassMember<Parent>>))
                .collect()
        }
    }

}

impl ClassCalls for Parent {
    fn call(&self, name: &str, arg: BoxedArg) -> BoxedReturn {
        self.members.get(name).expect(&format!("No such member: {}", name)).take(&self, arg)
    }
}

impl std::ops::Index<&str> for Parent
{
    type Output = BoxedMember<Self>;

    fn index(&self, i: &str) -> &BoxedMember<Self> {
        self.members.get(i).expect(&format!("No such method: {}", i))
    }
}

fn main() {
    let p = Parent::construct();
    call!(&p, "show_name", Box::new(5i32));
}
