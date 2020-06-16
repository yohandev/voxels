mod kind;

pub use kind::*;

/// represents a shader module
#[derive(Debug)]
pub struct Shader
{
    module: wgpu::ShaderModule,
    kind: ShaderKind
}

impl Shader
{
    /// compile a shader from its source GLSL code
    pub fn from_source(renderer: &crate::Renderer, kind: ShaderKind, src: &str) -> Self
    {
        use std::io::Cursor;
        use shaderc::*;

        let artifact = Compiler::new()
            .unwrap()
            .compile_into_spirv(src, kind.to_shaderc(), "shader.glsl", "main", None)
            .unwrap();
        
        let binary = Cursor::new(artifact.as_binary_u8());
        let shader = wgpu::read_spirv(binary).unwrap();
        
        Self
        {
            module: renderer.device.create_shader_module(&shader),
            kind
        }
    }

    /// load a shader from its path then compile it
    pub fn from_path() -> Self
    {
        todo!()
    }

    /// get the internal wgpu shader module
    pub(crate) fn module(&self) -> &wgpu::ShaderModule
    {
        &self.module
    }

    /// get what kind of shader self is
    pub fn kind(&self) -> ShaderKind
    {
        self.kind
    }
}