use crate::Storage;

/// trait for any struct that will be used as
/// an entity component.
pub trait Component: Default + Sized + 'static
{
    type Storage: Storage<Self>;
}