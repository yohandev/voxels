use crate::common::block::*;

impl<'a> UnpackedBlock<'a>
{
    /// should this block's face be culled by the other block?
    /// assumes `other` block touches `self` block on `face`
    pub fn cull(&self, other: &UnpackedBlock, face: BlockFace) -> bool
    {
        use BlockShapes::*;
        use shapes::*;

        match self.shape()
        {
            None => true,
            Cube => match other.shape()
            {
                None => false,                              // never cull
                Cube => true,                               // always cull
                Half => match other.variant().into()        // cull if fully covered
                {
                    HalfBlockVariants::North => match face  // opposing face = touching
                    {
                        BlockFace::South => true,
                        _ => false,
                    },
                    HalfBlockVariants::South => match face  // opposing face = touching
                    {
                        BlockFace::North => true,
                        _ => false,
                    },
                    HalfBlockVariants::West => match face   // opposing face = touching
                    {
                        BlockFace::East => true,
                        _ => false,
                    },
                    HalfBlockVariants::East => match face   // opposing face = touching
                    {
                        BlockFace::West => true,
                        _ => false,
                    },
                    HalfBlockVariants::Down => match face   // opposing face = touching
                    {
                        BlockFace::Up => true,
                        _ => false,
                    },
                    HalfBlockVariants::Up => match face     // opposing face = touching
                    {
                        BlockFace::Down => true,
                        _ => false,
                    },
                    HalfBlockVariants::NorthSouth => true,  // same as full blocks
                    HalfBlockVariants::WestEast => true,    // same as full blocks
                    HalfBlockVariants::DownUp => true,      // same as full blocks
                }
            },
            Half => todo!()
        }
    }
}