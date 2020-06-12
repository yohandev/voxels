use super::*;

/// rendering pipeline used to render merged cube meshes(chunks, basically)
pub struct ChunkPipeline
{
    
}

impl ChunkPipeline
{
    pub fn create(gfx: &Gfx)
    {
        use shaderc::ShaderKind::*;

        use super::uniforms::ModelViewProj;
        use super::vertices::ChunkVertex;

        let vs = gfx.ctx().create_shader_module(include_str!("../shaders/chunk.vert"), Vertex);
        let fs = gfx.ctx().create_shader_module(include_str!("../shaders/chunk.frag"), Fragment);

        gfx.ctx().create_pipeline()
            .with_binding_groups
            (&[
                gfx.mvp.bind_group_layout()
            ])
            .with_vertex_shader(vs)
            .with_fragment_shader(fs)
            .with_index_format(IndexFormat::Uint32)
            .with_vertex_format::<ChunkVertex>(ChunkVertex::ATTR)
            .build();
    }
}