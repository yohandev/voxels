use crate::marker::BufferData;

/// the uniform buffer is a bind-able shader resource,
/// which in GLSL looks like this:
/// ```glsl
/// layout(set=0, binding=0) uniform MyBuffer
/// {
///     mat4 u_buffer_data;
/// };
/// ```
/// it can store data common to multiple objects, pipelines,
/// and meshes(such as the view-projection matrix), or instance
/// data. the data can be updated through the ezgfx::Renderer
/// object, using ```renderer.update_buffer(&mut buffer, data);```.
pub struct Uniform<T: BufferData>
{
    buffer: wgpu::Buffer,
    size: usize,

    ty: std::marker::PhantomData<T>,
}

impl<T: BufferData> crate::Bind for Uniform<T>
{
    fn binding_type(&self) -> wgpu::BindingType
    {
        wgpu::BindingType::UniformBuffer { dynamic: false }
    }

    fn resource(&self) -> wgpu::BindingResource
    {
        wgpu::BindingResource::Buffer
        {
            buffer: &self.buffer,
            range: 0..self.size as wgpu::BufferAddress,
        }
    }
}

impl<T: BufferData> Uniform<T>
{
    /// create a new uniform buffer. this shouldn't be
    /// called directly.
    pub(crate) fn new(ctx: &crate::Renderer, data: T) -> Self
    {
        let buffer = ctx.device.create_buffer_with_data
        (
            bytemuck::cast_slice(&[data]),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST
        );
        let size = std::mem::size_of::<T>();
        let ty = Default::default();

        Self { buffer, size, ty }
    }
}