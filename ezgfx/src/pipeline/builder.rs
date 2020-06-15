/// a struct that builds a render pipeline.
/// see ezgfx::Renderer::pipeline() for details.
pub struct PipelineBuilder<'a>
{
    pub(crate) ctx: RendererRef<'a>,
    pub(crate) sets: Vec<ShaderResourceSet<'a>>
}

/// a temporary reference to a renderer.
/// lifetime must be valid for that of the
/// pipeline builder.
type RendererRef<'a> = &'a mut crate::Renderer;

/// a temporary reference to a bind group layout,
/// which describes a GLSL "set" mapping ```layout(set=N,binding=M)```.
type ShaderResourceSet<'a> = Option<&'a wgpu::BindGroupLayout>;

impl<'a> PipelineBuilder<'a>
{
    /// create a new pipeline builder, borrowing the renderer
    /// for its entire lifetime. this shouldn't be called
    /// directly.
    pub(crate) fn new(renderer: RendererRef<'a>) -> Self
    {
        Self
        {
            ctx: renderer,
            sets: vec![],
        }
    }

    /// start defining a new set, then chain to that set's
    /// bindings. in a GLSL shader:
    /// ```glsl
    /// layout(set = N, binding = A) uniform texture2D t_diffuse;
    /// layout(set = N, binding = B) uniform sampler s_diffuse;
    /// ```
    /// this ```PipelineBuilder::set(N)``` call would "push" a scope to
    /// edit the bindings, "A" and "B," within the set "N."
    pub fn set(mut self, slot: u32) -> crate::BindGroupBuilder<'a>
    {
        // make room for slot
        if self.sets.len() < slot as usize
        {
            self.sets.resize(slot as usize, None);
        }

        // return bind group editor
        crate::BindGroupBuilder::new(self, slot)
    }

    pub fn build(self) -> wgpu::RenderPipeline
    {
        let layout = self.ctx.device.create_pipeline_layout
        (
            &wgpu::PipelineLayoutDescriptor
            {
                bind_group_layouts: self.sets
                    .iter()
                    .map(|e| e.expect("pipelines cannot have gaps in its sets!"))
                    .collect::<Vec<&'a wgpu::BindGroupLayout>>()
                    .as_slice()
            }
        );

        self.ctx.device.create_render_pipeline
        (
            &wgpu::RenderPipelineDescriptor
            {
                layout: &layout,
                vertex_stage: wgpu::ProgrammableStageDescriptor
                {
                    module: todo!(),
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor
                {
                    module: todo!(),
                    entry_point: "main",
                }),
                rasterization_state: Some(wgpu::RasterizationStateDescriptor
                {
                    front_face: todo!(),
                    cull_mode: todo!(),
                    depth_bias: 0,
                    depth_bias_slope_scale: 0.0,
                    depth_bias_clamp: 0.0
                }),
                primitive_topology: todo!(),
                color_states: &[wgpu::ColorStateDescriptor
                {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    alpha_blend: todo!(),
                    color_blend: todo!(),
                    write_mask: wgpu::ColorWrite::ALL,
                    
                }],
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor
                {
                    index_format: todo!(),
                    vertex_buffers: &[wgpu::VertexBufferDescriptor
                    {
                        stride: todo!(),
                        step_mode: wgpu::InputStepMode::Vertex,
                        attributes: todo!(),
                    }],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            }
        )
    }
}