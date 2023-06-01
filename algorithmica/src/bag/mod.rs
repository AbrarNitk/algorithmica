// Bag is a simple data structure that only support the adding
// elements an iterating through them
// It does not support the remove operation

pub mod r#box;
pub mod rc;

pub use r#box::Bag as BoxBag;
pub use rc::Bag as RcBag;
