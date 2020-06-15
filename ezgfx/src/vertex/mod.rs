/// represents a generic vertex that can be passed to
/// render pipelines.
pub trait Vertex: Sized
{
    /// vertex description to pass to tbe pipeline
    const DESC: &'static [VertexAttr];
}

/// an attribute or field in a vertex struct.
/// used for the Vertex::DESC declaration.
pub enum VertexAttr
{
    Ushort2,
    Ushort4,
    Short2,
    Short4,

    Float,
    Float2,
    Float3,
    Float4,

    Uint,
    Uint2,
    Uint3,
    Uint4,

    Int,
    Int2,
    Int3,
    Int4,
}

impl VertexAttr
{
    /// get the size, in bytes, of this vertex attribute
    pub fn size(&self) -> usize
    {
        match self
        {
            VertexAttr::Ushort2 => 4,
            VertexAttr::Ushort4 => 8,
            VertexAttr::Short2  => 4,
            VertexAttr::Short4  => 8,
            VertexAttr::Float   => 4,
            VertexAttr::Float2  => 8,
            VertexAttr::Float3  => 12,
            VertexAttr::Float4  => 16,
            VertexAttr::Uint    => 4,
            VertexAttr::Uint2   => 8,
            VertexAttr::Uint3   => 12,
            VertexAttr::Uint4   => 16,
            VertexAttr::Int     => 4,
            VertexAttr::Int2    => 8,
            VertexAttr::Int3    => 12,
            VertexAttr::Int4    => 16,
        } 
    }

    /// translate this enum to the wgpu one
    pub fn to_wgpu(&self) -> wgpu::VertexFormat
    {
        match self
        {
            VertexAttr::Ushort2 => wgpu::VertexFormat::Ushort2,
            VertexAttr::Ushort4 => wgpu::VertexFormat::Ushort4,
            VertexAttr::Short2 => wgpu::VertexFormat::Short2,
            VertexAttr::Short4 => wgpu::VertexFormat::Short4,
            VertexAttr::Float => wgpu::VertexFormat::Float,
            VertexAttr::Float2 => wgpu::VertexFormat::Float2,
            VertexAttr::Float3 => wgpu::VertexFormat::Float3,
            VertexAttr::Float4 => wgpu::VertexFormat::Float4,
            VertexAttr::Uint => wgpu::VertexFormat::Uint,
            VertexAttr::Uint2 => wgpu::VertexFormat::Uint2,
            VertexAttr::Uint3 => wgpu::VertexFormat::Uint3,
            VertexAttr::Uint4 => wgpu::VertexFormat::Uint4,
            VertexAttr::Int => wgpu::VertexFormat::Int,
            VertexAttr::Int2 => wgpu::VertexFormat::Int2,
            VertexAttr::Int3 => wgpu::VertexFormat::Int3,
            VertexAttr::Int4 => wgpu::VertexFormat::Int4,
        }
    }
}