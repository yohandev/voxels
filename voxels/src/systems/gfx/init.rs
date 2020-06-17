use ezgame::plugins::ezgfx::*;
use ezgame::legion::*;
use ezmath::*;

use crate::resources::gfx::*;

/// system that initializes the gfx resources
pub(super) fn system() -> Box<dyn Schedulable>
{
    let vertices: [SimpleVertex; 3] =
    [
        SimpleVertex { pos: float3::new(-0.5, -0.5, 0.0), col: float3::x() },
        SimpleVertex { pos: float3::new(0.5, -0.5, 0.0), col: float3::y()  },
        SimpleVertex { pos: float3::new(0.0, 0.5, 0.0), col: float3::z()   },
    ];
    let indices: [u16; 3] = [0, 1, 2];    

    const VS_SRC: &str = r"
        #version 450

        layout(location=0) in vec3 a_pos;
        layout(location=1) in vec3 a_col;

        layout(location=0) out vec3 v_col;

        void main()
        {
            v_col = a_col;

            gl_Position = vec4(a_pos, 1.0);
        }
    ";
    const FS_SRC: &str = r"
        #version 450

        layout(location=0) in vec3 v_col;
        layout(location=0) out vec4 f_color;

        void main()
        {
            f_color = vec4(v_col, 1.0);
        }
    ";

    SystemBuilder::new("gfx_init_system")
        // components
        .with_query(<Read<components::Graphics>>::query())
        // resources
        .write_resource::<&mut Option<SimpleGfxResources>>()
        // system
        .build(move |_, world, res, query|
        {
            if res.is_some()
            {
                println!("[warn] there's either two renderers or event was emited twice!")
            }

            for ctx in query.iter(world)
            {
                let vs = ctx.shader(ShaderKind::Vertex, VS_SRC);
                let fs = ctx.shader(ShaderKind::Fragment, FS_SRC);

                let geo = ctx.geometry(&vertices, &indices);

                let pipeline = ctx
                    .pipeline()
                        .vertex::<SimpleVertex>()
                        .index::<u16>()
                        .shader(&vs)
                        .shader(&fs)
                    .build();

                res.replace(SimpleGfxResources { vs, fs, geo, pipeline });
            }
        })
}