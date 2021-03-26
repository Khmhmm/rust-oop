use std::collections::HashMap;
use std::ops::Deref;
use oop_prelude::*;
use oop_macro::*;

// #[derive(Inherit)]
// pub struct Dog{}
// impl Dog{
//     fn f1(&self) {}
// }



struct Parent {
    methods: HashMap<String, Box<Method<Parent>>>,
    mut_methods: HashMap<String, Box<MethodMut<Parent>>>,
    f: i32
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
            ));
        v.push(
            ("get_len".to_string(),
                {
                    let f: BoxedRawMethod<Parent> =
                        to_box!(|p, _| { Box::new(p.f.clone()) })
                    ;
                    Box::new(Method::<Parent>::from(f))
                }
            ));

        v
    }

    fn get_mut_methods() -> Vec<(String, Box<MethodMut<Parent>>)> {
        let mut v = Vec::<(String, Box<MethodMut<Parent>>)>::new();
        v.push(
            ("set_f".to_string(),
                {
                    let f: BoxedRawMethodMut<Parent> =
                        to_box!(|p, f1| { p.f = format!("{:?}", f1).parse::<i32>().unwrap(); Box::new(()) })
                    ;
                    Box::new(MethodMut::<Parent>::from(f))
                }
            )
        );
        v
    }
}

impl ClassConstructor for Parent {

    fn construct() -> Parent {
        Parent{
            methods: Self::get_methods()
                .into_iter()
                .collect(),
            mut_methods: Self::get_mut_methods()
                .into_iter()
                .collect(),
            f: 1i32
        }
    }

}

impl ClassCalls for Parent {
    fn call(&self, name: &str, arg: BoxedArg) -> BoxedReturn {
        self.methods.get(name).expect(&format!("No such method: {}", name)).call(&self, arg)
    }

    fn call_mut(&mut self, name: &str, arg: BoxedArg) -> BoxedReturn {
        // self.mut_methods.get_mut(name).expect(&format!("No such mut method: {}", name)).call(&mut self, arg)
        to_box!(())
    }
}

// impl std::ops::Index<&str> for Parent
// {
//     type Output = BoxedMember<Self>;
//
//     fn index(&self, i: &str) -> &BoxedMember<Self> {
//         self.members.get(i).expect(&format!("No such method: {}", i))
//     }
// }

fn main() {
    let p = Parent::construct();
    // call!(&p, "set_f", Box::new(5i32));
    // println!( "{:?}", call!(&p, "get_len", Box::new(())) );

    // let d = Dog{};
    // Dog::inherent_call(&d, Box::new(()));
}
