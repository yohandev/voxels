use legion::prelude::*;

use crate::framework::*;
use crate::ezmath::*;
use crate::gfx::*;

pub struct Game
{
    ecs: Universe,

    worlds: Vec<World>,  // loaded worlds

    gfx: Option<Gfx>,   // graphics, loads on first render call
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

    fn on_render(&mut self, window: &mut Window)
    {
        if let Some(gfx) = &self.gfx
        {
            gfx.render(window.ctx());
        }
        else
        {
            self.gfx = Some(Gfx::new(window.ctx()))
        }
    }
}

impl Game
{
    pub fn new() -> Self
    {
        Self
        {
            ecs: Universe::new(),
            worlds: Vec::new(),
            gfx: None
        }
    }

    pub fn _ecs(&self) -> &Universe
    {
        &self.ecs
    }
}