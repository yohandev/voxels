/// hard-coded shapes of blocks
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[allow(dead_code)]
pub enum BlockShape
{
    /// a standard cube shape. variant
    /// affects texture
    Cube,
    /// stair shape. variant affects
    /// orientation
    Stair,
    /// half cube shape. variant affects
    /// orientation
    Half,
    /// billboard cross shape, for grass
    /// or flowers
    Cross,
    /// empty shape, usually for air. variant
    /// doesn't do anything(yet?)
    Empty,
    /// liquid shape, for water or lava, etc.
    /// variant is the flow level
    Liquid,
    /// mesh shape. this is rendered as-is
    /// and can't be optimized
    Mesh,
}