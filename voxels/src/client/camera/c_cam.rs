use ezmath::*;

/// perspective camera component
pub struct CCamera
{
    pub proj: float4x4,

    pub fov: f32,

    pub near: f32,
    pub far: f32,
}

impl CCamera
{
    /// create a new camera component
    pub fn new(fov: f32, near: f32, far: f32, width: f32, height: f32) -> Self
    {
        Self
        {
            proj: float4x4::perspective(width / height, fov, near, far),
            fov,
            near,
            far,
        }
    }

    /// recalculate the projection matrix for the given sizes
    pub fn resize(&mut self, width: f32, height: f32)
    {
        self.proj = float4x4::perspective(width / height, self.fov, self.near, self.far);
    }
}