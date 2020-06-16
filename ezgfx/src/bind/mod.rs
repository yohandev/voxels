mod uniform;
mod builder;
mod group;

pub use uniform::*;
pub use builder::*;
pub use group::*;

/// a bind-able shader resource, to be used within
/// ezgfx::Renderer::pipeline()'s pipeline builder.
/// there already are general implementations of this
/// trait, such as Texture, Uniform, and Sampler.
pub trait Bind
{
    /// get the wgpu binding type for this bindable
    /// resource. self shouldn't be needed, but is
    /// there to support boxing internally.
    fn binding_type(&self) -> wgpu::BindingType;

    /// get the actual wgpu resource for self.
    fn resource(&self) -> wgpu::BindingResource;
}