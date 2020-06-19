use super::*;

#[derive(Debug, Copy, Clone)]
#[repr(C)]
/// a camera's view-projection matrix
pub struct ViewProj(pub float4x4);

impl ViewProj
{
    pub const OPENGL_TO_WGPU_MATRIX: [f32; 16] =
    [
        1.0, 0.0, 0.0, 0.0,
        0.0, 1.0, 0.0, 0.0,
        0.0, 0.0, 0.5, 0.0,
        0.0, 0.0, 0.5, 1.0,
    ];

    pub fn create_uniform(ctx: &RenderCtx, slot: u32) -> Uniform<Self>
    {
        ctx.create_uniform()
            .with_binding_slot(slot)
            .with_visiblity(ShaderStage::VERTEX)
            .with_label("model_view_projection_uniform")
            .build(Self::default())
    }

    pub fn new(aspect: f32, fovy: f32, znear: f32, zfar: f32, pos: &float3) -> Self
    {
        use nalgebra::*;

        let view = Translation3::new(-pos.x, -pos.y, -pos.z).to_homogeneous();
        let proj = Perspective3::new(aspect, fovy, znear, zfar).into_inner();

        ViewProj(float4x4::from_row_slice(&Self::OPENGL_TO_WGPU_MATRIX) * proj * view)
    }
}

impl Default for ViewProj
{
    fn default() -> Self
    {
        Self(float4x4::identity())
    }
}

unsafe impl Pod for ViewProj {}
unsafe impl Zeroable for ViewProj {}
