use std::any::Any;

/// Represent an event, where only its type
/// matters and not the content of the structure 
pub trait Event: Any {}

impl<T: Any> Event for T { }