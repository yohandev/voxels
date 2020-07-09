use ezgame::window::evt;
use ezgame::ecs::*;
use ezgame::*;

/// updates the camera matrix on window resize
pub struct SCameraResize;

impl System for SCameraResize
{
    fn register(handlers: &mut Systems) 
    {
        handlers.insert::<evt::Resized>(0, Self::on_window_resized)
    }
}

impl SCameraResize
{
    fn on_window_resized(app: &mut Application)
    {
        let size = app
            .window()
            .as_ref()
            .unwrap()
            .inner_size();

        for registry in app.registries_mut()
        {
            for mut cam in <Write<super::CCamera>>::query().iter_mut(*registry)
            {
                cam.resize(size.width as f32, size.height as f32);
            }
        }
    }
}