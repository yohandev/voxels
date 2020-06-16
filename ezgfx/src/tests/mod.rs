use crate::*;

buffer_data!
(
    #[derive(Default)]
    struct ViewProjUniform
    {
        mat: [f32; 16]
    }
);

// no test, just making sure it doesn't err
fn test_pipeline_api()
{
    let renderer = Renderer::from_window(todo!());
    
    let mvp = renderer.uniform(ViewProjUniform::default());
    let mut mvp_bind = None;

    renderer.pipeline()
        .set(0)
            .binding(0, ShaderKind::Vertex, &mvp)
            .build(&mut mvp_bind)
        .build();
}