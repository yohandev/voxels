use crate::*;

buffer_data!
(
    #[derive(Default)]
    struct ViewProjUniform
    {
        mat: [f32; 16]
    }
);

buffer_data!
(
    #[derive(Default)]
    struct PosColVertex
    {
        pos: [f32; 3],
        col: [f32; 3],
    }
);

impl Vertex for PosColVertex
{
    const DESC: &'static [VertexAttr] = &[VertexAttr::Float3, VertexAttr::Float3];
}

// no test, just making sure it doesn't err
fn test_pipeline_api()
{
    let renderer = Renderer::from_window(todo!());
    
    // the shaders
    let vs = renderer.shader(ShaderKind::Vertex, "");
    let fs = renderer.shader(ShaderKind::Fragment, "");

    // the uniforms
    let mvp = renderer.bind_group
    (
        ShaderKind::Vertex,
        (renderer.uniform(ViewProjUniform::default()),),
    );
    
    // the pipeline
    let pipeline = renderer
        .pipeline()
            .bindings(&[&mvp])
            
            .shader(&vs)
            .shader(&fs)

            .vertex::<PosColVertex>()
            .index::<u32>()
        .build();
}