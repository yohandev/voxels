use crate::*;

/// system function, which does the actual logic of
/// a system and is returned in the `System::get_fn`
/// function.
pub type SysFn = Box<dyn legion::prelude::Schedulable>;

