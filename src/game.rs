use crate::framework::*;
use crate::ezmath::*;

pub struct Game
{
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
    }

    fn on_render(&self, window: &mut Window)
    {
        println!("rendering...")
    }
}

impl Game
{
    
}