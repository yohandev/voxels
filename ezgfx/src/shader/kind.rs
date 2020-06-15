/// Shader stages supported by ezgfx
pub enum ShaderKind
{
    Vertex,
    Fragment,
}

impl ShaderKind
{
    /// convert this shader kind enum to shaderc's
    pub(crate) fn to_shaderc(&self) -> shaderc::ShaderKind
    {
        match self
        {
            ShaderKind::Vertex => shaderc::ShaderKind::Vertex,
            ShaderKind::Fragment => shaderc::ShaderKind::Fragment,
        }
    }

    /// convert this shader kind enum to wgpu's
    pub(crate) fn to_wgpu(&self) -> wgpu::ShaderStage
    {
        match self
        {
            ShaderKind::Vertex => wgpu::ShaderStage::VERTEX,
            ShaderKind::Fragment => wgpu::ShaderStage::FRAGMENT,
        }
    }
}