mod macros;

pub use macros::*;

/// mark data as buffer data, to be used in uniform,
/// index, vertices, etc. buffers
pub trait BufferData: bytemuck::Pod + bytemuck::Zeroable + 'static { }