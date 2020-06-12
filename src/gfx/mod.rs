use bytemuck::*;
use wgpu::*;

use crate::framework::*;
use crate::ezmath::*;
use crate::voxel::*;
use crate::ecs::*;

pub mod pipelines;
pub mod uniforms;
pub mod vertices;
pub mod mesh;

pub struct Gfx
{
    mvp: Uniform<uniforms::ViewProj>,

    chunk_pip: RenderPipeline,
}

impl Gfx
{
    pub fn new(ctx: &RenderCtx) -> Self
    {
        use pipelines::*;
        use uniforms::*;

        let mvp = ViewProj::create_uniform(ctx, 0);
        let chunk_pip = ChunkPipeline::create(ctx, &mvp);

        Self { mvp, chunk_pip }
    }

    pub fn render(&self, world: &mut Dimension, ctx: &mut RenderCtx)
    {
        let frame = ctx.frame();

        let mut encoder = ctx.create_command_encoder("render encoder");
        {
            let mut pass = ctx
                .create_render_pass(&frame, &mut encoder)
                .with_clear(double4::new(0.1, 0.2, 0.3, 1.0))
                .build();
            
            pass.set_bind_group(0, &self.mvp.bind_group(), &[]);
            pass.set_pipeline(&self.chunk_pip);

            for chunk in world.chunks()
            {
                if let Some(chunk_mesh) = chunk.mesh()
                {
                    pass.set_vertex_buffer(0, chunk_mesh.vertex_buffer(), 0, 0);
                    pass.set_index_buffer(chunk_mesh.index_buffer(), 0, 0);

                    pass.draw_indexed(0..chunk_mesh.num_index() as u32, 0, 0..1);
                }
            }         
        }

        ctx.submit(encoder);
    }
}