use super::*;

fn test()
{
    let ctx = crate::Renderer::from_window(todo!());

    let common_layout = ctx.device.create_bind_group_layout
    (
        &wgpu::BindGroupLayoutDescriptor
        {
            bindings: &[],
            label: Some("hello"),
        }
    );

    let pipeline = ctx.pipeline()
        .set(0)
            .existing(&common_layout)
        //.set(1)
        .build();
}