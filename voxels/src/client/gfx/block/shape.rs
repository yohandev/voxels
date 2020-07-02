use crate::common::block::*;

impl BlockData
{
    /// returns whether a face(arg: `face`) on the target
    /// `t_*` block should be culled by the neighboring
    /// face on `n_*` block
    /// # arg info
    /// - `t_data`: target data, BlockData for the target block
    /// being culled
    /// - `n_data`: neighbor data, BlockData for the neighbor block
    /// in the direction of `face`
    /// - `t_var`: target variant, the block variant of the target
    /// block. this is necesarry because variants can affect shape
    /// - `n_var`: neighbor variant. same logic as above.
    pub fn culled(t_data: &Self, n_data: &Self, t_var: u16, n_var: u16, face: BlockFace)
    {
        // prelimenary checks
        if t_data.ty() == BlockType::Cube
    }
}