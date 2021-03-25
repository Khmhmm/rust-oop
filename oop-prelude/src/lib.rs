mod class;
mod method;
mod arg;
mod field;

pub use class::*;
pub use method::*;
pub use arg::*;
pub use field::*;

#[macro_export]
macro_rules! call {
    ($class: expr, $method: expr, $arg: expr) => {
        $class[$method].take(&$class, $arg)
    }
}

#[macro_export]
macro_rules! to_box {
    ($it: expr) => {
        Box::new($it)
    }
}
