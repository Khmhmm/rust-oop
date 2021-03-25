use std::collections::HashMap;
use std::ops::Deref;

macro_rules! call {
    ($class: expr, $method: expr, $arg: expr) => {
        $class[$method](&$class, $arg)
    }
}

trait Class{
    fn get_methods() -> Vec<(String, Method<Self>)> where Self: Sized;
    fn call(&self, name: &str, arg: BoxedArg) -> Option<()> where Self: Sized;
    fn take_method(&self, name: &str) -> Option< &Method<Self> > where Self: Sized;
}

impl std::ops::Index<&str> for Parent
{
    type Output = Method<Self>;
    
    fn index(&self, i: &str) -> &Method<Self> {
        self.take_method(i).expect(&format!("No such method: {}", i))
    }
}


trait MethodArg {
    fn arg(&self) -> ();
}

impl MethodArg for i32{
    fn arg(&self) -> () {}
}

type BoxedArg = Box<dyn MethodArg>;


struct Method<C: ?Sized> {
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

impl<C> Deref for Method<C> {
    type Target = Box<dyn Fn(&C, BoxedArg) -> Result<(), ()>>;
    fn deref(&self) -> &Box<dyn Fn(&C, BoxedArg) -> Result<(), ()>> {
        &self.inner
    }
}

struct Parent {
    methods: HashMap<String, Method<Parent>>
}

impl Parent {
    pub fn construct() -> Parent {
        Parent{
            methods: Self::get_methods().into_iter().collect()
        }
    }
    
}

impl Class for Parent {
    fn get_methods() -> Vec<(String, Method<Parent>)> {
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
    
    fn call(&self, name: &str, arg: BoxedArg) -> Option<()> {
        let method = self.methods.get(name)?;
        method.call(&self, arg).ok()
    }
    
    fn take_method(&self, name: &str) -> Option<&Method<Parent>> {
        self.methods.get(name)
    }
}

fn main() {
    let p = Parent::construct();
    call!(&p, "show_name", Box::new(5i32));
}