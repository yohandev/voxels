use image::*;

pub struct Texture
{
    view: wgpu::TextureView
}

impl crate::Bind for Texture
{
    fn binding_type(&self) -> wgpu::BindingType
    {
        wgpu::BindingType::SampledTexture
        {
            multisampled: false,
            dimension: wgpu::TextureViewDimension::D2,
            component_type: wgpu::TextureComponentType::Uint
        }
    }

    fn resource(&self) -> wgpu::BindingResource
    {
        wgpu::BindingResource::TextureView(&self.view)
    }
}

impl Texture
{
    /// create a new texture from an image file's bytes. this
    /// should not be called directly.
    pub(crate) fn new(ctx: &crate::Renderer, name: &str, img: ImageResult<DynamicImage>) -> Self
    {
        //let img = load_from_memory(bytes).unwrap();
        let img = img.unwrap();
        let dim = img.dimensions();
        let rgb = img.into_rgba();

        let size = wgpu::Extent3d
        {
            width: dim.0,
            height: dim.1,
            depth: 1,
        };

        let tex = ctx.device.create_texture                 // texture
        (
            &wgpu::TextureDescriptor
            {
                size,
                array_layer_count: 1,
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
                label: Some(name)
            }
        );
        let buf = ctx.device.create_buffer_with_data        // staging buffer
        (
            &rgb,
            wgpu::BufferUsage::COPY_SRC
        );

        let mut encoder = ctx.device.create_command_encoder // encoder
        (
            &wgpu::CommandEncoderDescriptor
            {
                label: Some("texture_buffer_copy_encoder")
            }
        );

        encoder.copy_buffer_to_texture                      // copy buffer
        (
            wgpu::BufferCopyView
            {
                buffer: &buf,
                offset: 0,
                bytes_per_row: 4 * dim.0,
                rows_per_image: dim.1
            },
            wgpu::TextureCopyView
            {
                texture: &tex,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO
            },
            size
        );
        ctx.queue.submit(&[ encoder.finish() ]);            // submit copy op

        Self
        {
            view: tex.create_default_view()                 // texture view
        }
    }
}