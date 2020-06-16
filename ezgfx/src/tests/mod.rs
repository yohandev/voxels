#![allow(unused_variables)]
#![allow(dead_code)]

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
    #[allow(unreachable_code)]
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

    // the geometry
    let geo = renderer.geometry(&[PosColVertex::default()], &[0u32]);
    
    // the pipeline
    let pipeline = renderer
        .pipeline()
            .bindings(&[&mvp])
            
            .shader(&vs)
            .shader(&fs)

            .vertex::<PosColVertex>()
            .index::<u32>()
        .build();

    renderer.render_pass
    (
        |_, mut pass|
        {
            pass.begin_clear(0.1, 0.2, 0.3, 1.0);

            pass.bind_group(0, &mvp);
            pass.geometry(&geo);

            pass.draw(0..1);
        }
    )
}