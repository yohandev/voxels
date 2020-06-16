use winit::window::Window;

use crate::*;

/// represents a render context tied to a window. it stores
/// everything needed to render to a surface, which in this
/// case, is a winit window.
#[derive(Debug)]
pub struct Renderer
{
    pub(crate) surface:    wgpu::Surface,
    pub(crate) device:     wgpu::Device,
    pub(crate) queue:      wgpu::Queue,
    pub(crate) sc_desc:    wgpu::SwapChainDescriptor,
    pub(crate) sc:         wgpu::SwapChain,
}

impl Renderer
{
    /// create a renderer from a winit window
    pub fn from_window(window: &Window) -> Self
    {
        futures::executor::block_on(Self::async_from_window(window))
    }

    async fn async_from_window(window: &Window) -> Self
    {
        let size = window.inner_size();                 // winit size

        let surface = wgpu::Surface::create             // surface
        (
            window
        );

        let aopt = wgpu::RequestAdapterOptions          // adapter options
        {
            power_preference: wgpu::PowerPreference::Default,
            compatible_surface: Some(&surface)
        };
        let adapter = wgpu::Adapter::request            // adapter
        (
            &aopt,
            wgpu::BackendBit::PRIMARY
        )
        .await
        .unwrap();

        let (device, queue) = adapter.request_device    // device, queue
        (
            &wgpu::DeviceDescriptor
            {
                extensions: wgpu::Extensions { anisotropic_filtering: false },
                limits: wgpu::Limits::default()
            }
        ).await;

        let sc_desc = wgpu::SwapChainDescriptor         // swap chain description
        {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo
        };
        let sc = device.create_swap_chain               // swap chain
        (
            &surface,
            &sc_desc
        );
        
        Self { surface, device, queue, sc_desc, sc, }
    }

    /// create a new pipeline using the pipeline builder.
    /// the rendering pipeline is what takes your buffers:
    /// vertices, indices, uniforms, etc. and maps them to
    /// screen-space, rasterizes them, etc. The pipeline
    /// builder seeks to map part of that pipeline in an easy
    /// way. 
    pub fn pipeline(&self) -> PipelineBuilder
    {
        PipelineBuilder::new(self)
    }

    /// create a brand new bind group, ensuing a new bind group layout
    /// and an actual bind group. see ezgfx::BindGroup for more.
    pub fn bind_group<T: BindGroupTuple>(&self, stage: ShaderKind, bindings: T) -> BindGroup<T>
    {
        BindGroup::new(self, stage, bindings)
    }

    /// create a new uniform buffer.
    pub fn uniform<T: crate::marker::BufferData>(&self, data: T) -> Uniform<T>
    {
        Uniform::<T>::new(self, data)
    }

    /// create new geometry. the slices passed in aren't consumed or
    /// stored to be retrieved later. you have to store them yourself
    /// to access them again, if needed.
    pub fn geometry<V: Vertex, I: Index>(&self, vertices: &[V], indices: &[I]) -> Geometry<V, I>
    {
        Geometry::new(self, vertices, indices)   
    }

    /// create a new shader module from its source code
    pub fn shader(&self, kind: ShaderKind, src: &str) -> Shader
    {
        Shader::from_source(self, kind, src)
    }

    /// get the next frame from the swapchain and begin a render pass,
    /// which encodes the rendering instructions and does the actual
    /// drawing.
    ///
    /// it takes in a rendering function, where all the rendering happens.
    pub fn render_pass<F>(&mut self, func: F) where F: FnOnce(&mut Renderer, RenderPass)
    {
        let frame = self.sc
            .get_next_texture()
            .expect("timeout getting texture");
        
        let mut encoder = self.device.create_command_encoder
        (
            &wgpu::CommandEncoderDescriptor { label: Some("render_pass_encoder") }
        );

        func(self, RenderPass::new(&mut encoder, &frame.view));

        self.queue.submit(&[ encoder.finish() ]);
    }
}