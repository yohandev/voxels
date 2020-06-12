use super::*;

impl RenderCtx
{
    pub fn create_pipeline<'a>(&'a self) -> PipelineBuilder
    {
        PipelineBuilder
        {
            ctx: self,
            bind_group_layouts: &[],
            vert_shader: None,
            frag_shader: None,
            winding: FrontFace::Ccw,
            culling: CullMode::Back,
            tex_format: TextureFormat::Bgra8UnormSrgb,
            colour_blend: BlendDescriptor::REPLACE,
            alpha_blend: BlendDescriptor::REPLACE,
            topology: PrimitiveTopology::TriangleList,
            index_format: IndexFormat::Uint16,
            vertex_attr: &[],
            vertex_stride: 0,
        }
    }
}

pub struct PipelineBuilder<'a>
{
    ctx: &'a RenderCtx,

    bind_group_layouts: &'a [&'a BindGroupLayout],

    vert_shader: Option<ShaderModule>,
    frag_shader: Option<ShaderModule>,

    winding: FrontFace,
    culling: CullMode,

    tex_format: TextureFormat,

    colour_blend: BlendDescriptor,
    alpha_blend: BlendDescriptor,

    topology: PrimitiveTopology,

    index_format: IndexFormat,
    vertex_attr: &'a [VertexAttributeDescriptor],
    vertex_stride: BufferAddress,
}

impl<'a> PipelineBuilder<'a>
{
    pub fn with_binding_groups(mut self, layouts: &'a [&BindGroupLayout]) -> Self
    {
        self.bind_group_layouts = layouts;
        self
    }

    pub fn with_vertex_shader(mut self, module: ShaderModule) -> Self
    {
        self.vert_shader = Some(module);
        self
    }

    pub fn with_fragment_shader(mut self, module: ShaderModule) -> Self
    {
        self.frag_shader = Some(module);
        self
    }

    pub fn with_face_winding(mut self, winding: FrontFace) -> Self
    {
        self.winding = winding;
        self
    }

    pub fn with_face_culling(mut self, culling: CullMode) -> Self
    {
        self.culling = culling;
        self
    }

    pub fn with_colour_format(mut self, format: TextureFormat) -> Self
    {
        self.tex_format = format;
        self
    }

    pub fn with_colour_blending(mut self, mode: BlendDescriptor) -> Self
    {
        self.colour_blend = mode;
        self
    }

    pub fn with_alpha_blending(mut self, mode: BlendDescriptor) -> Self
    {
        self.alpha_blend = mode;
        self
    }

    pub fn with_primitive_topology(mut self, topology: PrimitiveTopology) -> Self
    {
        self.topology = topology;
        self
    }

    pub fn with_index_format(mut self, format: IndexFormat) -> Self
    {
        self.index_format = format;
        self
    }

    pub fn with_vertex_attr(mut self, attr: &'a [VertexAttributeDescriptor]) -> Self
    {
        self.vertex_attr = attr;
        self
    }

    pub fn with_vertex_stride(mut self, stride: BufferAddress) -> Self
    {
        self.vertex_stride = stride;
        self
    }

    /// short-hand for both with_vertex_attr and with_vertex_stride
    pub fn with_vertex_format<T: Pod>(mut self, attr: &'a [VertexAttributeDescriptor]) -> Self
    {
        self.vertex_attr = attr;
        self.vertex_stride = std::mem::size_of::<T>() as BufferAddress;
        self
    }

    pub fn build(self) -> RenderPipeline
    {
        let vert_shader = self.vert_shader.expect("pipeline needs a vertex shader!");

        let pip_layout = self.ctx.device().create_pipeline_layout   // layout
        (
            &PipelineLayoutDescriptor
            {
                bind_group_layouts: self.bind_group_layouts
            }
        );

        let pipeline = self.ctx.device().create_render_pipeline     // pipeline
        (
            &RenderPipelineDescriptor
            {
                layout: &pip_layout,
                vertex_stage: ProgrammableStageDescriptor
                {
                    module: &vert_shader,
                    entry_point: "main"
                },
                fragment_stage: if let Some(frag_shader) = &self.frag_shader
                {
                    Some
                    (
                        ProgrammableStageDescriptor
                        {
                            module: frag_shader,
                            entry_point: "main"
                        }
                    )
                }
                else
                {
                    None
                },
                rasterization_state: Some
                (
                    RasterizationStateDescriptor
                    {
                        front_face: self.winding,
                        cull_mode: self.culling,
                        depth_bias: 0,
                        depth_bias_slope_scale: 0.0,
                        depth_bias_clamp: 0.0
                    }
                ),
                color_states: 
                &[
                    ColorStateDescriptor
                    {
                        format: self.tex_format,
                        color_blend: self.colour_blend,
                        alpha_blend: self.alpha_blend,
                        write_mask: ColorWrite::ALL
                    }
                ],
                primitive_topology: self.topology,
                depth_stencil_state: None,
                vertex_state: VertexStateDescriptor
                {
                    index_format: self.index_format,
                    vertex_buffers:
                    &[
                        VertexBufferDescriptor
                        {
                            stride: self.vertex_stride,
                            step_mode: InputStepMode::Vertex,
                            attributes: self.vertex_attr
                        }
                    ]
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false
            }
        );

        pipeline
    }
}