use super::*;

pub trait State
{
    fn on_update(&mut self, input: &Input);
    fn on_render(&mut self, window: &mut Window);
}