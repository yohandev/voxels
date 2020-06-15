mod uniform;

pub use uniform::*;

/// represents a bound-able resource, such as a uniform buffer,
/// a texture, or a sampler. 
pub trait Bind
{
    /// create a new instance of this type of bind-able resource,
    /// from a bind group layout. the layout needs to be retreived
    /// from the pipeline to which this resource will be bound.
    fn create(&self, layout: wgpu::BindGroupLayout) -> wgpu::BindingResource;

    /// get the type of self, to be used in wgpu::BindGroupLayout
    fn ty() -> wgpu::BindingType;

    /// create a bind group layout entry from this type o bind-able
    /// resource.
    fn layout(binding: u32, visibility: wgpu::ShaderStage) -> wgpu::BindGroupLayoutEntry
    {
        wgpu::BindGroupLayoutEntry
        {
            binding,
            visibility,
            ty: Self::ty(),    
        }
    }
}

/// a bind group is one or more bind-able resources that are "linked"
/// together. in a glsl shader, this would be a set.
/// ```glsl
/// layout(set = 0, binding = 0) texture2D t_diffuse;
/// layout(set = 0, binding = 1) sampler2D s_diffuse;
///
/// layout(set = 1, binding = 0) uniform Globals { /* ... */ };
/// ```
/// the texture and sampler, in this example, is one BindGroup, while
/// the uniform is another(the distinction is in the 'set' value).
pub struct BindGroup
{
    entries: [Option<wgpu::BindGroupLayoutEntry>; 32]
}

impl BindGroup
{
    /// set the type of resource at a binding slot. see the doc
    /// for ezgfx::BindGroup for what binding means. 
    pub fn set<T: Bind>(mut self, binding: u32, visibility: wgpu::ShaderStage) -> Self
    {
        self.entries[binding as usize] = Some(T::layout(binding, visibility));
        self
    }

    /// same as BindGroup::set, except it uses a pre-existing bind
    /// group layout entry. this is useful for shader resources common
    /// to multiple pipelines, such as a model-view-projection uniform.
    /// for such a case, you'd use Uniform::layout(0, VERTEX) and store
    /// the output. That output is what you'd pass into this function.
    pub fn set_existing(mut self, layout: wgpu::BindGroupLayoutEntry) -> Self
    {
        //self.entries[layout.binding as usize] = Some(layout);
        self
    }
}