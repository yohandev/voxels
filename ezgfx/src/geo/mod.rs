pub struct Geometry<V: crate::Vertex, I: crate::Index>
{
    v_buf: wgpu::Buffer,
    i_buf: wgpu::Buffer,

    v_len: usize,
    i_len: usize,

    v_ty: std::marker::PhantomData<V>,
    i_ty: std::marker::PhantomData<I>,
}