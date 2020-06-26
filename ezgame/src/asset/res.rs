pub struct RAssetPool<T>
{
    assets: Vec<Asset<T>>
}

pub struct Handle<T>
{
    id: usize,
    gen: usize,

    ty: std::marker::PhantomData<T>
}

struct Asset<T>
{
    gen: usize,
    data: T,
}