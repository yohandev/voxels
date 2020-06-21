use raw_window_handle::*;

use crate::*;

/// represents a render context tied to a window. it stores
/// everything needed to render to a surface, which in this
/// case, is a window.
#[derive(Debug)]
pub struct Renderer
{
    pub(crate) surface:    wgpu::Surface,
    pub(crate) device:     wgpu::Device,
    pub(crate) queue:      wgpu::Queue,
    pub(crate) sc_desc:    wgpu::SwapChainDescriptor,
    pub(crate) sc:         wgpu::SwapChain,

    depth: Texture,
}

impl Renderer
{
    /// create a renderer from a window
    pub fn from_window<T: HasRawWindowHandle>(window: &T, width: u32, height: u32) -> Self
    {
        futures::executor::block_on(Self::async_from_window(window, width, height))
    }

    async fn async_from_window<T: HasRawWindowHandle>(window: &T, width: u32, height: u32) -> Self
    {
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
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo
        };
        let sc = device.create_swap_chain               // swap chain
        (
            &surface,
            &sc_desc
        );

        let depth =                                     // depth texture
            Texture::new_depth(&device, width, height);

        Self { surface, device, queue, sc_desc, sc, depth }
    }

    /// resize the renderer
    pub fn resize(&mut self, width: u32, height: u32)
    {
        self.sc_desc.width = width;         // width
        self.sc_desc.height = height;       // height
        self.sc =                           // swap chain
        self.device.create_swap_chain
        (
            &self.surface,
            &self.sc_desc
        );
        self.depth =                        // depth texture
            Texture::new_depth(&self.device, width, height);
    }

    /// report the current width of the output texture of the
    /// renderer.
    pub fn width(&self) -> u32
    {
        self.sc_desc.width
    }

    /// report the current height of the output texture of the
    /// renderer.
    pub fn height(&self) -> u32
    {
        self.sc_desc.height
    }

    /// get the bind group behind the depth texture. this becomes
    /// invalid after each resize
    pub fn depth(&self) -> &Texture
    {
        &self.depth
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

    /// create a new bind group, "inspired" from the source bind group.
    /// data will be different, but the layout and compatiblities will be
    /// shared.
    pub fn clone_bind_group<T: BindGroupTuple>(&self, source: &BindGroup<T>, bindings: T) -> BindGroup<T>
    {
        source.clone(self, bindings)
    }

    /// create a new uniform buffer.
    pub fn uniform<T: crate::marker::BufferData>(&self, data: T) -> Uniform<T>
    {
        Uniform::<T>::new(self, data)
    }

    // update the data inside the uniform buffer
    pub fn update_uniform<T: BufferData>(&self, uniform: &Uniform<T>, data: T)
    {
        uniform.update(self, data);
    }

    /// create a new texture from bytes of an image file.
    /// ```rust
    /// renderer.texture_bytes("my_texture.png", include_bytes!("my_texture.png"));
    /// ```
    pub fn texture_bytes(&self, name: &str, bytes: &[u8]) -> Texture
    {
        Texture::new(self, name, image::load_from_memory(bytes))
    }

    /// create a new texture from the path of the image
    pub fn texture<T: AsRef<std::path::Path>>(&self, path: T) -> Texture
    {
        let path = path.as_ref();

        Texture::new(self, path.to_str().unwrap(), image::open(path))
    }

    pub fn sampler(&self, desc: &SamplerDesc) -> Sampler
    {
        Sampler::new(self, desc)
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

    /// get the next output texture from the swapchain
    pub fn frame(&mut self) -> Frame
    {
        Frame::new(self)
    }

    /// begin a render pass, which encodes the rendering instructions
    /// and does the actual drawing.
    /// it clear the output texture from the given frame with the given
    /// colour.
    pub fn render_pass<'a>(&self, frame: &'a mut Frame, clear: [f64; 4]) -> RenderPass<'a>
    {
        frame.encoder = Some(self.device.create_command_encoder
        (
            &wgpu::CommandEncoderDescriptor { label: Some("render_pass_encoder") }
        ));

        RenderPass::new(frame, clear)
    }

    /// submit a frame's current render pass for rendering
    pub fn submit(&self, frame: &mut Frame)
    {
        self.queue.submit(&[ frame.encoder.take().unwrap().finish() ]);
    }
}