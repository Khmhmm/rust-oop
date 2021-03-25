mod class;
mod method;
mod arg;

pub use class::*;
pub use method::*;
pub use arg::*;

#[macro_export]
macro_rules! call {
    ($class: expr, $method: expr, $arg: expr) => {
        $class[$method](&$class, $arg)
    }
}

#[macro_export]
macro_rules! to_box {
    ($it: expr) => {
        Box::new($it)
    }
}
