use super::*;

/// rendering pipeline used to render merged cube meshes(chunks, basically)
pub struct ChunkPipeline
{
    offset_bind_group_layout: BindGroupLayout,
    pipeline: RenderPipeline
}

impl ChunkPipeline
{
    pub fn create(ctx: &RenderCtx, mvp: &Uniform<uniforms::ViewProj>) -> Self
    {
        use shaderc::ShaderKind::*;

        use super::vertices::ChunkVertex;

        let vs = ctx.create_shader_module(include_str!("../shaders/chunk.vert"), Vertex);
        let fs = ctx.create_shader_module(include_str!("../shaders/chunk.frag"), Fragment);

        let offset_bind_group_layout = ctx.device().create_bind_group_layout
        (
            &BindGroupLayoutDescriptor
            {
                label: Some("chunk_offset_bind_group_layout"),
                bindings: &
                [
                    BindGroupLayoutEntry
                    {
                        binding: 0,
                        visibility: ShaderStage::VERTEX,
                        ty: BindingType::UniformBuffer { dynamic: false }
                    }
                ]
            }
        );

        let pipeline = ctx.create_pipeline()
            .with_binding_groups
            (&[
                mvp.bind_group_layout(),
                &offset_bind_group_layout,
            ])
            .with_vertex_shader(vs)
            .with_fragment_shader(fs)
            .with_index_format(IndexFormat::Uint32)
            .with_vertex_format::<ChunkVertex>(ChunkVertex::ATTR)
            .build();

        Self { pipeline, offset_bind_group_layout }
    }

    pub fn pipeline(&self) -> &RenderPipeline
    {
        &self.pipeline
    }

    pub fn create_chunk_offset_buffer(&self, ctx: &RenderCtx, pos: int3) -> (Buffer, BindGroup)
    {
        #[derive(Copy, Clone)]
        #[repr(C)]
        struct Data(int3);

        unsafe impl Pod for Data {}
        unsafe impl Zeroable for Data {}

        let buffer = ctx.create_buffer(&[Data(pos)], BufferUsage::UNIFORM);
        let bind = ctx.device().create_bind_group
        (
            &BindGroupDescriptor
            {
                layout: &self.offset_bind_group_layout,
                bindings:
                &[
                    Binding
                    {
                        binding: 0,
                        resource: BindingResource::Buffer
                        {
                            buffer: &buffer,
                            range: 0..std::mem::size_of::<int3>() as BufferAddress
                        }
                    }
                ],
                label: Some("chunk_offset_bind_group")
            }
        );

        (buffer, bind)
    }
}