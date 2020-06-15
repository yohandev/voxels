```rust
let mut tex = None;
let mut light = None;

let pip = renderer.pipeline()
    .set(0)
        .existing(&game.mvp_layout)
    .set(1)
        .binding::<Texture>(0, "img.png")
        .binding::<Sampler>(1, (Nearest, Nearest))
        .build(&mut tex)
    .set(2)
        .binding::<Uniform>(2, Light(20.0))
        .build(&mut light)
    .build();
```