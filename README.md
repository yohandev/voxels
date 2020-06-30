# `voxels` + `ez`

## `ezgfx`
### low level yet compact graphics API on top of wgpu
```rust
let ctx = Renderer::new(/* window ptr */);

buffer_data!
(
    struct PosColVertex
    {
        pos: [f32; 3],
        col: [f32; 4],
    }
);

impl Vertex for PosColVertex
{
    const DESC: &'static [VertexAttr] = &[VertexAttr::Float3, VertexAttr::Float4];
}

let vs = ctx.shader(ShaderKind::Vertex, include_str!("vert.glsl"));
let fs = ctx.shader(ShaderKind::Fragment, include_str!("frag.glsl"));

let pip = ctx
    .pipeline()
        .vertex::<PosColVertex>()
        .index::<u32>()
        .shader(&vs)
        .shader(&fs)
        .depth(true)
    .build();

let geo = ctx.geometry(&vertices[..], &indices[..]);

let mut frame = ctx.frame();
{
    let mut pass = ctx.render_pass(&mut frame, [0.1, 0.2, 0.3, 1.0]);
    
    pass.pipeline(&pip);
    pass.geometry(&geo);
    pass.draw(0..1);
}
ctx.submit(&mut frame);
```

## `ezgame`
### ecs game framework, powered by legion
```rust
use ezgame::time::*;
use ezgame::ecs::*;
use ezgame::*;

fn main()
{ 
    Application::run::<TestGame>();
}

struct TestGame;

impl Game for TestGame
{
    fn build(app: &mut Application) -> Self
    {
        app.systems().bundle::<GameBundle>();
        app.systems().insert::<SPhysics>();
        
        app.resources().insert(RGravity(-9.8));
        
        app.resources().insert
        (
            window::RWindowRequest::new()
                .width(600)
                .height(600)
                .title("voxels")
        );
    }
}

struct CPos(f32, f32, f32);
struct CVel(f32, f32, f32);

struct RGravity(f32);

struct SPhysics;

impl System for SPhysics
{
    const EVENT = evt::UPDATE;
    const ORDER = ord::HIGH;
    
    fn exe() -> SysFn
    {
        // begin...
        sys("physics_system")
        // components...
        .with_query(<(Write<CPos>, Write<CVel>)>::query())
        // resources...
        .read_resource::<RTime>()
        .read_resource::<RGravity>()
        // system...
        .build(|_, world, (r_time, r_grav), query)
        {
            for (mut pos, mut vel) in query.iter_mut(world)
            {
                vel.1 += r_grav.0;
            
                pos.0 += vel.0 * r_time.dt();
                pos.1 += vel.1 * r_time.dt();
                pos.2 += vel.2 * r_time.dt();
            }
        })
    }
}
```

## `ezmath`
### game linear algebra, powered by nalgebra
```rust
let foo = float3::new(0.0, 1.0, 2.0);
let bar = float4x4::identity();
let baz = float4x4::perspective(600.0 / 400.0, 45.0, 0.01, 100.0);
```

## `voxels`
### tester game for the `ez` suite
