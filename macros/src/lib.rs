pub use macros_derive::*;

pub trait IterObject<T> {
    fn to_params(self) -> Vec<T>;
}
