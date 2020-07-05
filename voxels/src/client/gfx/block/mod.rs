use ezmath::*;

use crate::client::gfx::{ ChunkMeshBuilder, ChunkVertex };
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

    /// meshes a given face of this block
    pub fn mesh(&self, mesh: &mut ChunkMeshBuilder, face: BlockFace)
    {
        match self.shape()
        {
            shapes::BlockShapes::None => {}                 // strictly no mesh
            shapes::BlockShapes::Cube =>                    // simple cube faces
            {
                gen_face(mesh, face, self.r_pos())
            }
            shapes::BlockShapes::Half =>                    // todo
            {
                todo!()
            }
        }
    }
}

/// creates a square face of a mesh using the chunk vertex
fn gen_face(mesh: &mut ChunkMeshBuilder, face: BlockFace, pos: int3)
{
    const POS: [[u32; 3]; 8] = 
    [
        [ 1 , 1 , 1 ],
        [ 0 , 1 , 1 ],
        [ 0 , 0 , 1 ],
        [ 1 , 0 , 1 ],
        [ 0 , 1 , 0 ],
        [ 1 , 1 , 0 ],
        [ 1 , 0 , 0 ],
        [ 0 , 0 , 0 ],
    ];

    const TRI: [[usize; 4]; 6] =
    [
        [ 4, 5, 6, 7 ],
        [ 0, 1, 2, 3 ],
        [ 1, 4, 7, 2 ],
        [ 5, 0, 3, 6 ],
        [ 3, 2, 7, 6 ],
        [ 5, 4, 1, 0 ],
    ];

    const IND: [u32; 6] =
    [
        0, 1, 2, 0, 2, 3
    ];

    for i in &TRI[face as usize]    // vertices
    {
        let x = POS[*i][0] + pos.x as u32;
        let y = POS[*i][1] + pos.y as u32;
        let z = POS[*i][2] + pos.z as u32;

        mesh.vert.push(ChunkVertex::new(&uint3::new(x, y, z), &uint2::new(pos.x as u32, pos.z as u32)));
    }

    let j = mesh.vert.len() as u32;
    for i in &IND                   // indices
    {
        mesh.ind.push(*i + j);
    }
}