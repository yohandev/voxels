mod sys;
mod evt;

pub use sys::*;
pub use evt::*;

// legion_core
pub use legion::prelude::
{
    // structs
    CommandBuffer,
    Entity,
    Query,
    Read,
    SubWorld,
    Tagged,
    TryRead,
    TryWrite,
    Universe,
    World,
    Write,

    // traits
    EntityStore,
    IntoQuery,

    // functions
    any,
    changed,
    component,
    passthrough,
    tag,
    tag_value,
};

// legion_systems
pub use legion::prelude::
{
    // structs
    BitSet,
    Resources,
    SystemBuilder,
    //Schedule,

    // traits
    ResourceSet,
    // Schedulable,
    
    //Runnable,
    //System,
};

pub use legion::borrow::
{
    Ref as CmpRef,
    RefMut as CmpRefMut,
};