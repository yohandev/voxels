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

use pipelines::*;
use uniforms::*;

pub struct Gfx
{
    mvp: Uniform<ViewProj>,

    pub chunk_pip: ChunkPipeline,

    pub cam_pos: float3,
}

impl Gfx
{
    pub fn new(ctx: &RenderCtx) -> Self
    {
        let mvp = ViewProj::create_uniform(ctx, 0);
        let chunk_pip = ChunkPipeline::create(ctx, &mvp);

        Self { mvp, chunk_pip, cam_pos: float3::new(0.0, 0.0, -1.0) }
    }

    pub fn render(&self, world: &mut Dimension, window: &mut Window)
    {
        use uniforms::*;

        let width = window.width() as f32;
        let height = window.height() as f32;

        let ctx = window.ctx();
        let frame = ctx.frame();

        self.mvp.update_data(ctx, ViewProj::new(width / height, 45.0f32.to_radians(), 0.01, 100.0, &self.cam_pos));
        
        let mut encoder = ctx.create_command_encoder("render encoder");
        {
            let mut pass = ctx
                .create_render_pass(&frame, &mut encoder)
                .with_clear(double4::new(0.1, 0.2, 0.3, 1.0))
                .build();
            
            //let offsets = world.chunks().map(|chunk| ChunkModel(*chunk.pos())).collect::<Vec<_>>();

            //self.model.bind_group().
            //*self.model = ChunkModel::create_storage_buffer(ctx, 0);
            //self.model.update_data(ctx, offsets);

            pass.set_bind_group(0, &self.mvp.bind_group(), &[]);
            pass.set_pipeline(&self.chunk_pip.pipeline());


            //let mut offsets = Vec::<ChunkModel>::with_capacity(world.chunks().len());
            //let mut offsets = ChunkModel::default();
            for (i, chunk) in world.chunks().enumerate()
            {
                let i = i as u32;

                //offsets.push(ChunkModel::new(*chunk.pos()));
                if let Some(chunk_mesh) = &chunk.gfx
                {
                    pass.set_bind_group(1, &chunk_mesh.2, &[]);

                    pass.set_index_buffer(chunk_mesh.0.index_buffer(), 0, 0);
                    pass.set_vertex_buffer(0, chunk_mesh.0.vertex_buffer(), 0, 0);

                    pass.draw_indexed(0..chunk_mesh.0.num_index() as u32, 0, i..(i + 1));
                }
            }
        }

        ctx.submit(encoder);
    }
}