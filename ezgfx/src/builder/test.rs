use super::*;

struct Uniform<T>
{
    buffer: wgpu::Buffer,
    len: usize,

    data: std::marker::PhantomData<T>
}

impl<T> ShaderResource for Uniform<T>
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
            range: 0..(self.len * std::mem::size_of::<T>()) as wgpu::BufferAddress
        }
    }
}

fn test()
{
    let ctx = crate::Renderer::from_window(todo!());

    let common_layout = ctx.device.create_bind_group_layout
    (
        &wgpu::BindGroupLayoutDescriptor
        {
            bindings: &[],
            label: Some("hello"),
        }
    );

    let uni = Uniform::<f32> { buffer: todo!(), len: 0, data: todo!()  };

    let mut uni_bind = None;

    let pipeline = ctx.pipeline()
        .set(0)
            .existing(&common_layout)
        .set(1)
            .binding(0, wgpu::ShaderStage::VERTEX, &uni)
            .build(&mut uni_bind)
        .build();
}