use ezgame::gfx::*;
use ezmath::*;

/// shared graphic resources for chunks
pub type RGraphicsChunk = Option
<(
    Shader,         // shared vertex shader
    Shader,         // shared fragment shader

    ChunkPosBind,   // shared chunk position uniform
    Pipeline,       // shared rendering pipeline
)>;

/// the geometry and position uniform of a chunk
pub struct ChunkMesh
{
    pub geo: ChunkGeometry,
    pub pos: ChunkPosBind,
}

/// geometry of a given chunk
pub type ChunkGeometry = Geometry<ChunkVertex, u32>;
/// position bind group of a chunk
pub type ChunkPosBind = BindGroup<(Uniform<ChunkPosition>,)>;

buffer_data!
(
    /// compressed vertex layout
    /// 6 bit x -> 0-64 local position
    /// 6 bit y -> 0-64 local position
    /// 6 bit z -> 0-64 local position
    /// 7 bit u -> 128 x 128 texture atlas
    /// 7 bit v -> 128 x 128 texture atlas
    ///
    /// remarks
    ///     - position can support bigger chunks up to 63, but it's
    ///       unlikely the 32 size will change
    ///     - texture atlas can have at most ~16,000 square textures,
    ///       which is good enough for the current block limit of 4096
    ///     - only simple square blocks are supported by this vertex
    ///         - stairs and half-blocks which need fractional positions,
    ///           (ie. x = 20.5) can't be represented in this model.
    pub struct ChunkVertex
    {
        compressed: u32
    }
);

buffer_data!
(
    /// uniform for a chunk position
    #[derive(Default)]
    pub struct ChunkPosition
    {
        pub position: int3
    }
);

impl Vertex for ChunkVertex
{
    const DESC: &'static [VertexAttr] = &[VertexAttr::Uint];
}

impl ChunkVertex
{
    pub fn new(pos: &uint3, tex: &uint2) -> Self
    {
        debug_assert!(pos.x <= 63 && pos.y <= 63 && pos.z <= 63, "vertex position needs to be localized 0..64!");
        debug_assert!(tex.x <= 127 && tex.y <= 127, "texture coord cannot exceed 0..128 range!");

        Self
        {
            compressed: (pos.x << 26)
            | (pos.y << 20)
            | (pos.z << 14)
            | (tex.x << 7)
            | (tex.y)
        }
    }

    #[allow(dead_code)]
    pub fn x(&self) -> u32
    {
        self.compressed >> 26
    }

    #[allow(dead_code)]
    pub fn y(&self) -> u32
    {
        (self.compressed >> 20) & 0b0011_1111
    }

    #[allow(dead_code)]
    pub fn z(&self) -> u32
    {
        (self.compressed >> 14) & 0b0011_1111
    }

    #[allow(dead_code)]
    pub fn u(&self) -> u32
    {
        (self.compressed >> 7) & 0b0111_1111
    }

    #[allow(dead_code)]
    pub fn v(&self) -> u32
    {
        self.compressed & 0b0111_1111
    }
}