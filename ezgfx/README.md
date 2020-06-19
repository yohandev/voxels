```rust
let tex = Texture::new("assets/img.png");
let smp = Sampler::new(Nearest, Nearest);

let light = Uniform::new(Light(20, 30));

let mut tex_bind = None;
let mut light_bind = None;

let pip = renderer.pipeline()
    .set(0)
        .existing(&game.mvp_layout)
    .set(1)
        .binding(0, Texture, tex)
        .binding(1, Fragment, smp)
        .build(&mut tex_bind)
    .set(2)
        .binding(2, Vertex, light)
        .build(&mut light_bind)
    .build();
```

# experimental api
```rust
// creates a ezgfx::BindGroup::<Texture, Sampler>
//
// behind the scenes the BindGroup struct looks like:
// <T0: Bind, T1: Bind>(pub T0, pub T1, Rc<wgpu::BindGroupLayout>, wgpu::BindGroup)
//
// The generic tuple is generated for sizes 1-32
//
// The bind group's elements, texture and sampler, can be retrieved through
// my_bind_group.0 and my_bind_group.1
//
// Bind group can be "recreated"(same layout, compatable with all pipelines) using
// renderer.clone_bind_group(&my_bind_group, Texture::load("another/img.png"), Sampler::new(Nearest, Nearest));
let my_bind_group = renderer.bind_group(Fragment, Texture::load("path/img.png"), Sampler::new(Nearest, Nearest));

// changes the ezgfx::Pipeline
let my_pip = renderer
    .pipeline()
        .bind_groups(&[&my_bind_group])

        .shader(&my_vert_shader)
        .shader(&my_frag_shader)

        .vertex::<PosColVertex>()
        .index::<u32>()
    .build();

// clone a bind group
// now, both my_bg_clone and my_bind_group can be bound to set#0 of my_pip
let my_bg_clone = renderer.clone_bind_group(&my_bind_group, Texture::load("another/img.png"), Sampler::new(Nearest, Nearest));
```