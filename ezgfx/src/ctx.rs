use winit::window::Window;
use wgpu::*;

/// represents a render context tied to a window. it stores
/// everything needed to render to a surface, which in this
/// case, is a winit window.
pub struct Renderer
{
    surface:    Surface,
    dev:        Device,
    queue:      Queue,
    sc_desc:    SwapChainDescriptor,
    sc:         SwapChain,
}

impl Renderer
{
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

        Self { surface, dev, queue, sc_desc, sc }
    }

    // pub fn create_pipeline<T: crate::Pipeline>()
    // {

    // }
}