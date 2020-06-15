use super::*;

pub struct Uniform
{

}

impl Bind for Uniform
{
    fn ty() -> wgpu::BindingType
    {
        wgpu::BindingType::UniformBuffer { dynamic: false }
    }

    fn create(&self, layout: wgpu::BindGroupLayout) -> wgpu::BindingResource
    {
        todo!()
    }
}