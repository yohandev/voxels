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
    pub(in super::super) async fn from_window(window: &Window) -> Self
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

    pub fn submit(&self, encoder: CommandEncoder)
    {
        self.queue.submit(&[ encoder.finish() ]);
    }
}