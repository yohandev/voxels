use crate::*;

/// geometry is a container for both a vertex and index
/// buffer. it can be bound to the render pass, then
/// drawn.
pub struct Geometry<V: Vertex, I: Index>
{
    pub(crate) v_buf: wgpu::Buffer,
    pub(crate) i_buf: wgpu::Buffer,

    pub(crate) v_len: usize,
    pub(crate) i_len: usize,

    v_ty: std::marker::PhantomData<V>,
    i_ty: std::marker::PhantomData<I>,
}

impl<V: Vertex, I: Index> Geometry<V, I>
{
    /// create new geometry from its vertices and indices.
    /// this should not be called directly
    pub(crate) fn new(ctx: &Renderer, vertices: &[V], indices: &[I]) -> Self
    {
        let v_buf = ctx.device.create_buffer_with_data
        (
            bytemuck::cast_slice(vertices),
            wgpu::BufferUsage::VERTEX
        );
        let i_buf = ctx.device.create_buffer_with_data
        (
            bytemuck::cast_slice(indices),
            wgpu::BufferUsage::INDEX
        );

        let v_len = vertices.len();
        let i_len = indices.len();

        let v_ty = Default::default();
        let i_ty = Default::default();

        Self { v_buf, i_buf, v_len, i_len, v_ty, i_ty }
    }

    /// get the number of indices in this geometry
    pub fn num_indices(&self) -> usize
    {
        self.i_len
    }

    /// get the number of vertices in this geometry
    pub fn num_vertices(&self) -> usize
    {
        self.v_len
    }
}