use winit::window::Window;
use wgpu::*;

/// represents a render context tied to a window. it stores
/// everything needed to render to a surface, which in this
/// case, is a winit window.
#[derive(Debug)]
pub struct Renderer
{
    pub(crate) surface:    Surface,
    pub(crate) device:     Device,
    pub(crate) queue:      Queue,
    pub(crate) sc_desc:    SwapChainDescriptor,
    pub(crate) sc:         SwapChain,
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
        let size = window.inner_size();             // winit size

        let surface = Surface::create               // surface
        (
            window
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

        let (device, queue) = adapter.request_device   // device, queue
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
        let sc = device.create_swap_chain              // swap chain
        (
            &surface,
            &sc_desc
        );

        Self { surface, device, queue, sc_desc, sc }
    }

    /// create a new pipeline using the pipeline builder.
    /// the rendering pipeline is what takes your buffers:
    /// vertices, indices, uniforms, etc. and maps them to
    /// screen-space, rasterizes them, etc. The pipeline
    /// builder seeks to map part of that pipeline in an easy
    /// way. 
    pub fn pipeline(&self) -> crate::PipelineBuilder
    {
        crate::PipelineBuilder::new(self)
    }

    /// create a new uniform buffer.
    pub fn uniform<T: crate::marker::BufferData>(&self, data: T) -> crate::Uniform<T>
    {
        crate::Uniform::<T>::new(self, data)
    }
}