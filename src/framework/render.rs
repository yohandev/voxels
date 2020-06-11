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