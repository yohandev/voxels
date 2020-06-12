use crate::framework::*;
use crate::ezmath::*;
use crate::voxel::*;
use crate::gfx::*;
use crate::ecs::*;

pub struct Game
{
    ecs: Universe,

    worlds: Vec<Dimension>, // loaded worlds

    gfx: Option<Gfx>,       // graphics, loads on first render call
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
            if let Some(world) = self.worlds.first_mut()
            {
                gfx.render(world, window.ctx());
            }
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
        let uni = Universe::new();

        Self
        {
            worlds: vec![Dimension::new(uni.create_world())],
            ecs: uni,
            gfx: None
        }
    }

    pub fn _ecs(&self) -> &Universe
    {
        &self.ecs
    }
}