mod kind;

pub use kind::*;

/// represents a shader module
pub struct Shader
{
    pub(crate) module: wgpu::ShaderModule
}

impl Shader
{
    /// compile a shader from its source GLSL code
    pub fn from_source(renderer: crate::Renderer, kind: ShaderKind, src: &str) -> Self
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
            module: renderer.device.create_shader_module(&shader)
        }
    }

    /// load a shader from its path then compile it
    pub fn from_path() -> Self
    {
        todo!()
    }
}