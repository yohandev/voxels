use ezgame::plugins::ezgfx::resources::*;
use ezgame::plugins::ezgfx::ShaderKind;
use ezgame::legion::*;
use ezmath::*;

use crate::resources::gfx::*;

/// system that initializes the gfx resources
pub(super) fn system() -> Box<dyn Schedulable>
{
    let vertices: [SimpleVertex; 3] =
    [
        SimpleVertex { pos: float3::new(-0.5, -0.5, -1.0), col: float3::x() },
        SimpleVertex { pos: float3::new(0.5, -0.5, -1.0), col: float3::y()  },
        SimpleVertex { pos: float3::new(0.0, 0.5, -1.0), col: float3::z()   },
    ];
    let indices: [u16; 3] = [0, 1, 2];    

    const VS_SRC: &str = r"
        #version 450

        layout(location=0) in vec3 a_pos;
        layout(location=1) in vec3 a_col;

        layout(location=0) out vec3 v_col;

        layout(set=0, binding=0) uniform Globals
        {
            mat4 u_view_proj;
        };

        void main()
        {
            v_col = a_col;

            gl_Position = u_view_proj * vec4(a_pos, 1.0);
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
        // resources
        .write_resource::<SimpleGfxResources>()
        .read_resource::<Renderer>()
        // system
        .build(move |_, _, (res, ctx), _|
        {
            let ctx = ctx.as_ref().unwrap();
            
            let vs = ctx.shader(ShaderKind::Vertex, VS_SRC);
            let fs = ctx.shader(ShaderKind::Fragment, FS_SRC);

            let geo = ctx.geometry(&vertices, &indices);

            let vp = ctx.uniform(ViewProjUniform::default());
            let vp = ctx.bind_group(ShaderKind::Vertex, (vp,));

            let pipeline = ctx
                .pipeline()
                    .bindings(&[&vp])
                    .vertex::<SimpleVertex>()
                    .index::<u16>()
                    .shader(&vs)
                    .shader(&fs)
                .build();

            res.replace(SimpleGfxResourcesStruct { vs, fs, geo, pipeline, vp });
        })
}