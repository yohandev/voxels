#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[allow(dead_code)]
/// a type of block, used for optimial chunk meshing and
/// collision detection. different block types are meshed
/// separately as they use different type of vertices.
pub enum BlockType
{
    /// not meshed or bounded, fully transparent
    Gas,
    /// standard block, where mesh/bounding box
    /// fills the entire space of the voxel. since
    /// vertices for this kind of block snap to the
    /// 32x32x32 grid, vertex size can be optimized.
    Cube,
    /// block composed of one or more rectangular prisms,
    /// aligned to the voxel grid with 4 degress of
    /// precision. ie. a vertex can either be at 0,
    /// 0.25, 0.5, or 0.75 along a voxel in each axes.
    /// this is especially useful for simple "half" blocks
    /// like slabs or stairs, while keeping the vertex
    /// cost relatively low. the 'half' name is analogous
    /// to floating point numbers varying in precision:
    /// `half`, `single`, `double`.
    Half,
    /// block composed of one or more rectangular prisms,
    /// aligned to the voxel grid with 32 degress of
    /// precision. ie. a vertex can either be at 0, 0.03125,
    /// 0.0625, ..., or 0.9375 along a voxel in each axes.
    /// this is useful for complex "fraction" blocks that
    /// still fit the blocky theme, like voxel statues or
    /// snow. the vertex cost is still relatively low,
    /// since each position number isn't full float numbers.
    /// the 'single' name is analogous to floating point
    /// numbers varying in precision: `half`, `single`,
    /// `double`.
    Single,
    /// a liquid block type. it's the same as BlockType::Half,
    /// but with predefined shapes and rendered seperately for
    /// transparency.
    Liquid,
    /// wild card block, where anything goes. meshes
    /// aren't(can't be) optimized to cull faces and position
    /// dimensions are float's. this is the heaviest and least
    /// optimized type of block but can have anything.
    Mesh,
}