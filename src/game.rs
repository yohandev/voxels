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
    }

    fn on_render(&self, window: &mut Window)
    {
        //println!("rendering...")
    }
}

impl Game
{
    
}