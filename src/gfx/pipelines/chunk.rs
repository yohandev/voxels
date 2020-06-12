use super::*;

/// rendering pipeline used to render merged cube meshes(chunks, basically)
pub struct ChunkPipeline
{

}

impl ChunkPipeline
{
    pub fn create(ctx: &RenderCtx)
    {
        use shaderc::ShaderKind::*;

        use super::uniforms::ModViewProjUniform;
        use super::vertices::ChunkVertex;

        let vs = ctx.create_shader_module(include_str!("../shaders/chunk.vert"), Vertex);
        let fs = ctx.create_shader_module(include_str!("../shaders/chunk.frag"), Fragment);

        ctx.create_pipeline()
            .with_binding_groups
            (&[
                &ModViewProjUniform::bind_group_layout(ctx, 0)
            ])
            .with_vertex_shader(vs)
            .with_fragment_shader(fs)
            .with_index_format(IndexFormat::Uint32)
            .with_vertex_format::<ChunkVertex>(ChunkVertex::ATTR);
    }
    // pub fn create(gfx: &Gfx)
    // {
    //     let pip_layout = gfx.ctx.device().create_pipeline_layout    // pipeline layout
    //     (
    //         &PipelineLayoutDescriptor
    //         {
    //             bind_group_layouts: &[]
    //         }
    //     );

    //     let pipeline = gfx.ctx.device().create_render_pipeline      // pipeline
    //     (
    //         &RenderPipelineDescriptor
    //         {
    //             layout: &pip_layout,
    //             vertex_stage: ProgrammableStageDescriptor
    //             {
    //                 module: &shader.modules[0],
    //                 entry_point: "main"
    //             },
    //             fragment_stage: Some(ProgrammableStageDescriptor
    //             {
    //                 module: &shader.modules[1],
    //                 entry_point: "main"
    //             }),
    //             rasterization_state: Some(RasterizationStateDescriptor
    //             {
    //                 front_face: FrontFace::Ccw,
    //                 cull_mode: CullMode::Back,
    //                 depth_bias: 0,
    //                 depth_bias_slope_scale: 0.0,
    //                 depth_bias_clamp: 0.0
    //             }),
    //             color_states: 
    //             &[
    //                 ColorStateDescriptor
    //                 {
    //                     format: format,
    //                     color_blend: BlendDescriptor::REPLACE,
    //                     alpha_blend: BlendDescriptor::REPLACE,
    //                     write_mask: ColorWrite::ALL
    //                 }
    //             ],
    //             primitive_topology: PrimitiveTopology::TriangleList,
    //             depth_stencil_state: None,
    //             vertex_state: VertexStateDescriptor
    //             {
    //                 index_format: I::format(),
    //                 vertex_buffers:
    //                 &[
    //                     VertexBufferDescriptor
    //                     {
    //                         stride: mem::size_of::<V>() as BufferAddress,
    //                         step_mode: InputStepMode::Vertex,
    //                         attributes: V::desc()
    //                     }
    //                 ]
    //             },
    //             sample_count: 1,
    //             sample_mask: !0,
    //             alpha_to_coverage_enabled: false
    //         }
    //     );
    // }
}