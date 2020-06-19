use ezgame::plugins::ezgfx::resources::*;
use ezgame::plugins::ezgfx::ShaderKind;
use ezgame::legion::*;

use crate::resources::gfx::*;

pub fn system() -> Box<dyn Schedulable>
{
    const VS_SRC: &str = r"
        #version 450

        layout(location = 0) in uint a_compressed;
        
        layout(location = 0) out vec2 v_uv;
        
        layout(set = 0, binding = 0) uniform ViewProjection
        {
            mat4 u_view_proj;
        };
        // layout(set = 1, binding = 0) uniform ChunkOffset
        // {
        //     ivec3 u_offset;
        // };
        
        void main()
        {
            float x = float(a_compressed >> 26);// + float(u_offset.x);
            float y = float((a_compressed >> 20) & 63);// + float(u_offset.y);
            float z = float((a_compressed >> 14) & 63);// + float(u_offset.z);
        
            float u = float((a_compressed >> 7) & 127) / 128;
            float v = float(a_compressed & 127) / 128;
        
            v_uv = vec2(u, v);
            gl_Position = u_view_proj * vec4(x, y, z, 1.0);
        }
    ";
    const FS_SRC: &str = r"
        #version 450

        layout(location = 0) in vec2 v_uv;
        
        layout(location = 0) out vec4 f_color;
        
        void main()
        {
            f_color = vec4(v_uv.x, v_uv.y, 0.0, 1.0);
        }
    ";

    SystemBuilder::new("gfx_init_system")
        // resources
        .write_resource::<ChunkGfxResources>()
        .read_resource::<SimpleGfxResources>()
        .read_resource::<Renderer>()
        // system
        .build(move |_, _, (chunk_res, global_res, ctx), _|
        {
            let ctx = ctx.as_ref().unwrap();
            
            let vs = ctx.shader(ShaderKind::Vertex, VS_SRC);
            let fs = ctx.shader(ShaderKind::Fragment, FS_SRC);

            let vp = &global_res.as_ref().unwrap().vp;

            let pipeline = ctx
                .pipeline()
                    .bindings(&[vp])
                    .vertex::<ChunkVertex>()
                    .index::<u32>()
                    .shader(&vs)
                    .shader(&fs)
                .build();

            let chunk_geo = vec![];

            chunk_res.replace(ChunkGfxRes { vs, fs, pipeline, chunk_geo });
        })
}