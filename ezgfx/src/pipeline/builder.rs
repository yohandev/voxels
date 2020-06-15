/// a struct that builds a render pipeline.
/// see ezgfx::Renderer::pipeline() for details.
#[derive(Debug)]
pub struct PipelineBuilder<'a>
{
    pub(crate) ctx: RendererRef<'a>,
    pub(crate) sets: Vec<ShaderResourceSet<'a>>,

    vert_shader: Option<&'a crate::Shader>,
    frag_shader: Option<&'a crate::Shader>,

    vert_format: Option<Vec<wgpu::VertexAttributeDescriptor>>,
    vert_stride: usize,

    index_format: Option<wgpu::IndexFormat>,

    settings: PipelineSettings,
}

#[derive(Debug, Default, Clone)]
/// all the many pipeline settings, condensed
/// into a single default-able struct
struct PipelineSettings
{
    winding: FaceWinding,
    culling: Culling,
}

/// a temporary reference to a renderer.
/// lifetime must be valid for that of the
/// pipeline builder.
type RendererRef<'a> = &'a crate::Renderer;

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

            vert_shader: None,
            frag_shader: None,

            vert_format: None,
            vert_stride: 0,

            index_format: None,

            settings: Default::default(),
        }
    }

    /// add a shader module to this render pipeline.
    /// you have to manually add the vertex AND fragment
    /// shaders both.
    pub fn shader(mut self, module: &'a crate::Shader) -> Self
    {
        match module.kind()
        {
            crate::ShaderKind::Vertex =>   self.vert_shader = Some(module),
            crate::ShaderKind::Fragment => self.frag_shader = Some(module),
        }
        self
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

    /// override the default face winding mode.
    /// default: counter clockwise
    pub fn winding(mut self, mode: FaceWinding) -> Self
    {
        self.settings.winding = mode;
        self
    }

    /// override the default face culling mode.
    /// default: back
    pub fn culling(mut self, mode: Culling) -> Self
    {
        self.settings.culling = mode;
        self
    }

    /// set the vertex type used by this pipeline.
    /// this is a necesarry attribute and will panic
    /// on PipelineBuilder::build() if not set.
    pub fn vertex<T: crate::Vertex>(mut self) -> Self
    {
        let mut offset = 0;
        let mut location = 0;
        self.vert_format = Some
        (
            T::DESC
                .iter()
                .map(|attr|
                {
                    // current attribute info
                    let off = offset;
                    let loc = location;

                    // step for next attribute
                    offset   += attr.size();
                    location += 1;

                    // declare current attribute
                    wgpu::VertexAttributeDescriptor
                    {
                        offset: off as wgpu::BufferAddress,
                        format: attr.to_wgpu(),
                        shader_location: loc,
                    }
                })
                .collect::<Vec<_>>()
        );
        self.vert_stride = offset;

        debug_assert_eq!
        (
            offset,
            std::mem::size_of::<T>(),
            "vertex declaration and vertex struct don't match size! did you forget to include #[repr(C)]?"
        );

        self
    }

    /// set the index type used by this pipeline.
    /// this is a necesarry attribute and will panic
    /// on PipelineBuilder::build() if not set.
    /// the two options provided by ezgfx are u16
    /// and u32.
    pub fn index<T: crate::Index>(mut self) -> Self
    {
        self.index_format = Some(T::DESC);
        self
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
                    module: self.vert_shader
                        .expect("pipeline must have a vertex shader!")
                        .module(),
                    entry_point: "main",
                },
                fragment_stage: Some(wgpu::ProgrammableStageDescriptor
                {
                    module: self.vert_shader
                        .expect("pipeline must have a fragment shader!")
                        .module(),
                    entry_point: "main",
                }),
                rasterization_state: Some(wgpu::RasterizationStateDescriptor
                {
                    front_face: self.settings.winding.to_wgpu(),
                    cull_mode: self.settings.culling.to_wgpu(),
                    depth_bias: 0,
                    depth_bias_slope_scale: 0.0,
                    depth_bias_clamp: 0.0
                }),
                primitive_topology: wgpu::PrimitiveTopology::TriangleList,
                color_states: &[wgpu::ColorStateDescriptor
                {
                    format: wgpu::TextureFormat::Bgra8UnormSrgb,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                    
                }],
                depth_stencil_state: None,
                vertex_state: wgpu::VertexStateDescriptor
                {
                    index_format: self.index_format
                        .expect("pipeline must have an index declaration!"),
                    vertex_buffers: &
                    [
                        wgpu::VertexBufferDescriptor
                        {
                            stride: self.vert_stride as wgpu::BufferAddress,
                            step_mode: wgpu::InputStepMode::Vertex,
                            attributes: self.vert_format
                                .expect("pipeline must have a vertex declaration!")
                                .as_slice(),
                        }
                    ],
                },
                sample_count: 1,
                sample_mask: !0,
                alpha_to_coverage_enabled: false,
            }
        )
    }
}

/// what vertex direction determines the front face?
/// default: counter clockwise
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum FaceWinding
{
    Clockwise,
    CounterClockwise
}

/// face culling mode. the front face is determined on the
/// rasterizer's face winding.
/// default: back
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Culling
{
    Front,
    Back,
    None,
}

impl FaceWinding
{
    /// translate this enum to a wgpu one
    fn to_wgpu(&self) -> wgpu::FrontFace
    {
        match self
        {
            FaceWinding::Clockwise => wgpu::FrontFace::Cw,
            FaceWinding::CounterClockwise => wgpu::FrontFace::Ccw,
        }
    }
}

impl Culling
{
    /// translate this enum to a wgpu one
    fn to_wgpu(&self) -> wgpu::CullMode
    {
        match self
        {
            Culling::Front => wgpu::CullMode::Front,
            Culling::Back => wgpu::CullMode::Back,
            Culling::None => wgpu::CullMode::None,
        }
    }
}

impl Default for FaceWinding
{
    fn default() -> Self
    {
        Self::CounterClockwise
    }
}

impl Default for Culling
{
    fn default() -> Self
    {
        Self::Back
    }
}