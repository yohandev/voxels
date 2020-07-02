use ezmath::*;

/// block face enum, in global coordinates.
/// that means a block's right face, for example
/// is always right no matter how it's rotated.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[allow(dead_code)]
#[repr(u8)]
pub enum BlockFace
{
    /// -z
    North,
    /// +z
    South,
    /// -x
    West,
    /// +x
    East,
    /// -y
    Down,
    /// +y
    Up,
}

impl BlockFace
{
    /// get the normalized direction
    /// of this block face, that is,
    /// the up block face yields a
    /// <0, 1, 0> vector. adding a
    /// block face's normal vector
    /// to a block's position yields
    /// the position of the block
    /// adjacent to that face.
    /// # example
    /// ```rust
    /// let foo = int3::new(0, 3, 1);
    /// let baz = foo + BlockFace::North::normal();
    ///
    /// // baz is the block that touches foo's
    /// // north face.
    /// ```
    pub fn normal(self) -> int3
    {
        match self
        {
            BlockFace::North => int3::new(0, 0, -1),
            BlockFace::South => int3::new(0, 0,  1),
            BlockFace::West  => int3::new(-1, 0, 0),
            BlockFace::East  => int3::new(1, 0,  0),
            BlockFace::Down  => int3::new(0, -1, 0),
            BlockFace::Up    => int3::new(0, 1,  0),
        }
    }

    /// get the block face opposite to this
    /// one
    pub fn opposite(self) -> BlockFace
    {
        match self
        {
            BlockFace::North => BlockFace::South,
            BlockFace::South => BlockFace::North,
            BlockFace::West  => BlockFace::East,
            BlockFace::East  => BlockFace::West,
            BlockFace::Down  => BlockFace::Up,
            BlockFace::Up    => BlockFace::Down,
        }
    }
}