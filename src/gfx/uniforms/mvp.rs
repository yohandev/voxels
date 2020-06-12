use super::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ModelViewProj
{
    mat: float4,
}

impl ModelViewProj
{
    
}

unsafe impl Pod for ModelViewProj {}
unsafe impl Zeroable for ModelViewProj {}
