use bytemuck::*;
use shaderc::*;
use wgpu::*;

use super::*;

pub struct RenderCtx
{
    surface: Surface,
    adapter: Adapter,
    dev: Device,
    queue: Queue,
    sc_desc: SwapChainDescriptor,
    sc: SwapChain,
}

impl RenderCtx
{
    pub(super) async fn from_window(window: &Window) -> Self
    {
        let size = window.winit.inner_size();       // winit size

        let surface = Surface::create               // surface
        (
            &window.winit
        );

        let aopt = RequestAdapterOptions            // adapter options
        {
            power_preference: PowerPreference::Default,
            compatible_surface: Some(&surface)
        };
        let adapter = Adapter::request              // adapter
        (
            &aopt,
            BackendBit::PRIMARY
        )
        .await
        .unwrap();

        let (dev, queue) = adapter.request_device   // device, queue
        (
            &DeviceDescriptor
            {
                extensions: Extensions { anisotropic_filtering: false },
                limits: Limits::default()
            }
        ).await;

        let sc_desc = SwapChainDescriptor           // swap chain description
        {
            usage: TextureUsage::OUTPUT_ATTACHMENT,
            format: TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: PresentMode::Fifo
        };
        let sc = dev.create_swap_chain              // swap chain
        (
            &surface,
            &sc_desc
        );

        Self { surface, adapter, dev, queue, sc_desc, sc }
    }

    pub fn resize(&mut self, size: uint2)
    {
        self.sc_desc.width = size.x;                // width
        self.sc_desc.height = size.y;               // height
        self.sc = self.dev.create_swap_chain        // swap chain
        (
            &self.surface,
            &self.sc_desc
        );
    }

    pub fn frame(&mut self) -> SwapChainOutput
    {
        self.sc
            .get_next_texture()
            .expect("timeout getting render texture from the swapchain!")
    }

    pub fn device(&self) -> &Device
    {
        &self.dev
    }

    pub fn queue(&self) -> &Queue
    {
        &self.queue
    }

    // -- utility functions --

    pub fn create_command_encoder(&self, name: &str) -> CommandEncoder
    {
        self.dev.create_command_encoder
        (
            &CommandEncoderDescriptor
            {
                label: Some(name)
            }
        )
    }

    pub fn create_render_pass<'a>(&'a self, frame: &'a SwapChainOutput, encoder: &'a mut CommandEncoder) -> RenderPassBuilder
    {
        RenderPassBuilder
        {
            ctx: self,

            frame,
            encoder,

            load: LoadOp::Clear,
            store: StoreOp::Store,
            
            clear: double4::new(0.0, 0.0, 0.0, 1.0),
            depth: None
        }
    }

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

    pub fn create_shader_module(&self, src: &str, kind: ShaderKind) -> ShaderModule
    {
        use std::io::Cursor;

        let artifact = Compiler::new()
            .unwrap()
            .compile_into_spirv(src, kind, "shader.glsl", "main", None)
            .unwrap();
        
        let binary = Cursor::new(artifact.as_binary_u8());
        let shader = read_spirv(binary).unwrap();
        
        self.dev.create_shader_module(&shader)
    }

    pub fn create_buffer<T: Pod>(&self, data: T, usage: BufferUsage) -> Buffer
    {
        self.dev.create_buffer_with_data(cast_slice(&[data]), usage)
    }

    pub fn create_uniform_buffer<T: Pod>(&self, data: T) -> Buffer
    {
        self.create_buffer(data, BufferUsage::UNIFORM | BufferUsage::COPY_DST)
    }

    pub fn submit(&self, encoder: CommandEncoder)
    {
        self.queue.submit(&[ encoder.finish() ]);
    }
}

pub struct RenderPassBuilder<'a>
{
    ctx: &'a RenderCtx,

    frame: &'a SwapChainOutput,
    encoder: &'a mut CommandEncoder,

    load: LoadOp,
    store: StoreOp,

    clear: double4,
    depth: Option<RenderPassDepthStencilAttachmentDescriptor<'a>>,
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

impl<'a> RenderPassBuilder<'a>
{
    pub fn with_load(mut self, load: LoadOp) -> Self
    {
        self.load = load;
        self
    }

    pub fn with_store(mut self, store: StoreOp) -> Self
    {
        self.store = store;
        self
    }

    pub fn with_clear(mut self, clear: double4) -> Self
    {
        self.clear = clear;
        self
    }

    // TODO pub fn with_depth(mut self, ..)

    pub fn build(self) -> RenderPass<'a>
    {
        self.encoder.begin_render_pass
        (
            &RenderPassDescriptor
            {
                color_attachments:
                &[
                    RenderPassColorAttachmentDescriptor
                    {
                        attachment: &self.frame.view,
                        resolve_target: None,
                        load_op: self.load,
                        store_op: self.store,
                        clear_color: Color
                        {
                            r: self.clear.x,
                            g: self.clear.y,
                            b: self.clear.z,
                            a: self.clear.w
                        }
                    }
                ],
                depth_stencil_attachment: self.depth
            }
        )
    }
}