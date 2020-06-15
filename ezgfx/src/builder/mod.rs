mod test;

pub struct PipelineBuilder<'a>
{
    ctx: &'a mut crate::Renderer,
    sets: Vec<Set<'a>>
}

pub struct BindGroupBuilder<'a>
{
    parent: PipelineBuilder<'a>,

    res: Vec<(Box<&'a dyn ShaderResource>, u32, wgpu::ShaderStage)>,
    set: u32
}

type Set<'a> = Option<&'a wgpu::BindGroupLayout>;

impl<'a> PipelineBuilder<'a>
{
    pub(crate) fn new(ctx: &'a mut crate::Renderer) -> Self
    {
        Self
        {
            ctx,
            sets: Default::default(),
        }
    }

    pub fn set(mut self, slot: u32) -> BindGroupBuilder<'a>
    {
        // make room for slot
        if self.sets.len() < slot as usize
        {
            self.sets.resize(slot as usize, None);
        }

        // return bind group editor
        BindGroupBuilder
        {
            parent: self,

            res: Default::default(),
            set: slot as u32,
        }
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

pub trait ShaderResource
{
    fn binding_type(&self) -> wgpu::BindingType;
    fn resource(&self) -> wgpu::BindingResource;
}

impl<'a> BindGroupBuilder<'a>
{
    pub fn existing(mut self, layout: &'a wgpu::BindGroupLayout) -> PipelineBuilder<'a>
    {
        self.parent.sets[self.set as usize] = Some(layout);
        self.parent
    }

    pub fn binding(mut self, slot: u32, stage: wgpu::ShaderStage, res: &'a dyn ShaderResource) -> Self
    {
        self.res.push((Box::new(res), slot, stage));
        self
    }

    pub fn build(mut self, out: &'a mut Option<(wgpu::BindGroupLayout, wgpu::BindGroup)>) -> PipelineBuilder<'a>
    {
        let mut layout_entries = Vec::with_capacity(self.res.len());
        let mut bind_entries = Vec::with_capacity(self.res.len());

        for res in self.res
        {
            layout_entries.push(wgpu::BindGroupLayoutEntry
            {
                binding: res.1,
                visibility: res.2,
                ty: res.0.binding_type(),
            });
            bind_entries.push(wgpu::Binding
            {
                binding: res.1,
                resource: res.0.resource(),
            });
        }

        let layout = self.parent.ctx.device.create_bind_group_layout
        (
            &wgpu::BindGroupLayoutDescriptor
            {
                bindings: layout_entries.as_slice(),
                label: None,
            }
        );
        let bind = self.parent.ctx.device.create_bind_group
        (
            &wgpu::BindGroupDescriptor
            {
                layout: &layout,
                bindings: bind_entries.as_slice(),
                label: None,
            }   
        );

        *out = Some((layout, bind));

        self.parent.sets[self.set as usize] = Some(&out.as_ref().unwrap().0);
        self.parent
    }
}