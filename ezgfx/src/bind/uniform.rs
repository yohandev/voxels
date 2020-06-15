pub struct Uniform<T: Sized>
{
    buffer: wgpu::Buffer,
    len: usize,

    ty: std::marker::PhantomData<T>,
}

impl<T: Sized> Uniform<T>
{
    
}