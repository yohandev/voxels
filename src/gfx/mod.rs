use bytemuck::*;
use wgpu::*;

use crate::framework::*;
use crate::ezmath::*;

pub mod pipelines;
pub mod uniforms;
pub mod vertices;

pub struct Gfx
{
    _mvp: Uniform<uniforms::ModelViewProj>,

    _chunk_pip: pipelines::ChunkPipeline,
}

impl Gfx
{
    pub fn new(ctx: &RenderCtx) -> Self
    {
        use pipelines::*;
        use uniforms::*;

        let mvp = ModelViewProj::create_uniform(ctx, 0);
        let chunk_pip = ChunkPipeline::create(ctx, &mvp);

        Self { _mvp: mvp, _chunk_pip: chunk_pip }
    }

    pub fn render(&self, ctx: &mut RenderCtx)
    {
        let frame = ctx.frame();

        let mut encoder = ctx.create_command_encoder("render encoder");
        {
            let pass = ctx
                .create_render_pass(&frame, &mut encoder)
                .with_clear(double4::new(0.1, 0.2, 0.3, 1.0))
                .build();
            // -- render operations --
            
        }

        ctx.submit(encoder);
    }
}