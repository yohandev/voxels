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

    pub(crate) frame:      Option<wgpu::SwapChainOutput>,
    pub(crate) pass_cmds:  Option<wgpu::CommandEncoder>,
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
        
        let frame = None;                               // swap chain output
        let pass_cmds = None;                           // render pass command encoder

        Self { surface, device, queue, sc_desc, sc, frame, pass_cmds }
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

    /// create a new uniform buffer.
    pub fn uniform<T: crate::marker::BufferData>(&self, data: T) -> Uniform<T>
    {
        Uniform::<T>::new(self, data)
    }

    /// create new geometry
    pub fn geometry<V: Vertex, I: Index>(&self, vertices: &[V], indices: &[I]) -> Geometry<V, I>
    {
        Geometry::new(self, vertices, indices)   
    }

    /// get the next frame for rendering
    pub fn frame(&mut self)
    {
        self.frame = Some               // frame texture
        (
            self.sc
                .get_next_texture()
                .expect("timeout getting texture")
        );
        
        self.pass_cmds = Some           // render pass encoder
        (
            self.device.create_command_encoder
            (
                &wgpu::CommandEncoderDescriptor { label: Some("render_pass_encoder") }
            )
        );
    }

    // /// create a new render pass. this must be called after the
    // /// ezgfx::Renderer::frame() call
    // pub fn render_pass(&mut self, clear: [f64; 4]) -> RenderPass
    // {
    //     RenderPass::new(self, clear)
    // }
}