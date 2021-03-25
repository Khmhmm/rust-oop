use std::collections::HashMap;

trait MethodArg {
    fn arg(&self) -> ();
}

impl MethodArg for i32{
    fn arg(&self) -> () {}
}

type BoxedArg = Box<dyn MethodArg>;


struct Method<C> {
    inner: Box<dyn Fn(&C, BoxedArg) -> Result<(),()>>
}
impl<C> Method<C>{
    #[inline]
    pub fn call(&self, c: &C, arg: BoxedArg) -> Result<(), ()> {
        (self.inner)(c, arg)
    }
}

impl<C> From<Box<dyn Fn(&C, BoxedArg) -> Result<(), ()>>> for Method<C> {
    fn from( f: Box<dyn Fn(&C, BoxedArg) -> Result<(), ()>> ) -> Self {
        Method {
            inner: f
        }
    }
}

struct Parent {
    methods: HashMap<String, Method<Parent>>
}

impl Parent {
    pub fn get_methods() -> Vec<(String, Method<Parent>)> {
        let mut v = Vec::<(String, Method<Parent>)>::new();
        v.push(
            ("show_name".to_string(), 
                {
                    let f: Box<dyn Fn(&Parent, BoxedArg) -> Result<(), ()>> = 
                        Box::new(|p, _| { println!("hello world"); Ok(()) })
                    ;
                    f.into()    
                }
            )
        );
        
        v
            
    }
    pub fn construct() -> Parent {
        Parent{
            methods: Self::get_methods().into_iter().collect()
        }
    }
    
    pub fn call(&self, name: &str, arg: BoxedArg) -> Option<()> {
        let method = self.methods.get(name)?;
        method.call(&self, arg).ok()
    }
}

fn main() {
    let p = Parent::construct();
    p.call("show_name", Box::new(5i32));
}