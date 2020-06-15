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
        .binding(0, Fragment, tex)
        .binding(1, Fragment, smp)
        .build(&mut tex_bind)
    .set(2)
        .binding(2, Vertex, light)
        .build(&mut light_bind)
    .build();
```