mod half;

pub use half::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(dead_code)]
pub enum BlockShapes
{
    /// marks a block as having no shape/mesh/bounding
    /// box. it's air, basically
    None,
    /// standard block, where mesh/bounding box
    /// fills the entire space of the voxel. since
    /// vertices for this kind of block snap to the
    /// 32x32x32 grid, vertex size is the most
    /// optimized
    Cube,
    /// half block shape, which can be rotated and stacked.
    /// it uses the 4th degree precesion vertex model,
    /// which is 1 byte more than the vertex model used by
    /// the cube shape
    Half,
}