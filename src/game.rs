use legion::prelude::*;

use crate::framework::*;
use crate::ezmath::*;

pub struct Game
{
    ecs: Universe,

    worlds: Vec<World>,  // loaded worlds
}

impl State for Game
{
    fn on_update(&mut self, input: &Input)
    {
        println!("updating @ {}ms of delta", input.delta_time_f32() * 1000.0);

        // quick test
        if input.key_pressed(KeyCode::Space)
        {
            println!("key pressed!");
        }
        if input.key_released(KeyCode::Space)
        {
            println!("key released!");
        }

        //self.ecs.create_world().resources.
    }

    fn on_render(&self, window: &mut Window)
    {
        let ctx = window.ctx();
        let frame = ctx.frame();

        let mut encoder = ctx.create_command_encoder("render encoder");
        {
            let pass = ctx
                .create_render_pass(&frame, &mut encoder)
                .with_clear(double4::new(0.1, 0.2, 0.3, 1.0))
                .build();
            // -- render operations --
            
        }

        ctx.submit(encoder);
    }
}

impl Game
{
    pub fn new() -> Self
    {
        Self
        {
            ecs: Universe::new(),
            worlds: Vec::new()
        }
    }

    pub fn ecs(&self) -> &Universe
    {
        &self.ecs
    }
}