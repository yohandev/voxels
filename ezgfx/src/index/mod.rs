/// represents a generic index that can be passed to
/// render pipelines.
pub trait Index: Sized
{
    /// the wgpu translated type of index
    const DESC: wgpu::IndexFormat;
}

impl Index for u16
{
    const DESC: wgpu::IndexFormat = wgpu::IndexFormat::Uint16;
}

impl Index for u32
{
    const DESC: wgpu::IndexFormat = wgpu::IndexFormat::Uint32;
}