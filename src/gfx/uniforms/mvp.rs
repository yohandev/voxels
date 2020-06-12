use super::*;

pub struct ModelViewProjectionUniform
{
    mat: float4,
    buf: Buffer,
}

impl ModelViewProjectionUniform
{
    pub fn create(ctx: &RenderCtx, mat: float4) -> Self
    {
        #[derive(Debug, Copy, Clone)]
        #[repr(C)]
        struct Data(float4);

        unsafe impl Pod for Data {}
        unsafe impl Zeroable for Data {}

        let buf = ctx.create_uniform_buffer(Data(mat));

        Self
        {
            mat,
            buf,
        }
    }
}