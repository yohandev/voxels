mod builder;

pub use builder::*;

/// a render pipeline
pub struct Pipeline(pub(crate) wgpu::RenderPipeline);