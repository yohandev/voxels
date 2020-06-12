use super::*;

/// rendering pipeline used to render merged cube meshes(chunks, basically)
pub struct ChunkPipeline
{
    pip: RenderPipeline
}

impl ChunkPipeline
{
    pub fn create(ctx: &RenderCtx, mvp: &Uniform<uniforms::ModelViewProj>) -> Self
    {
        use shaderc::ShaderKind::*;

        use super::vertices::ChunkVertex;

        let vs = ctx.create_shader_module(include_str!("../shaders/chunk.vert"), Vertex);
        let fs = ctx.create_shader_module(include_str!("../shaders/chunk.frag"), Fragment);

        let pip = ctx.create_pipeline()
            .with_binding_groups
            (&[
                mvp.bind_group_layout()
            ])
            .with_vertex_shader(vs)
            .with_fragment_shader(fs)
            .with_index_format(IndexFormat::Uint32)
            .with_vertex_format::<ChunkVertex>(ChunkVertex::ATTR)
            .build();
        
            Self { pip }
    }
}