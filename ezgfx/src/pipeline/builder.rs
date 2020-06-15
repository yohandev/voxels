pub struct PipelineBuilder<'a>
{
    ctx: &'a crate::Renderer,

    sets: Vec<BindGroup>,

}

type BindGroup = Option<Vec<Option<wgpu::BindGroupLayoutEntry>>>;

impl<'a> PipelineBuilder<'a>
{
    /// set the type, if any, of bind-able resource at this
    /// pipeline's set and binding. this mirror's GLSL layout
    /// construct:
    ///```glsl
    /// layout(set = 0, binding = 0) texture2D t_diffuse;
    /// layout(set = 0, binding = 1) sampler2D s_diffuse;
    ///
    /// layout(set = 1, binding = 0) uniform Globals { /* ... */ };
    /// ``` 
    /// internally, this creates a new wgpu::BindGroupLayout. it's
    /// not stored so whatever resource declared through this method
    /// is unique to this pipeline. use PipelineBuilder::resource_existing
    /// for resources shared across pipelines. 
    pub fn resource<T: crate::Bind>(set: u32, binding: u32) //-> wgpu::BindGroupLayout
    {

    }

    pub fn resource_existing(set: u32, layout: wgpu::BindGroupLayout)
    {

    }
}