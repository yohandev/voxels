use ezgame::time::evt;
use ezgame::ecs::*;
use ezgame::*;

/// system that prints the frames per second
pub struct SDebugFps;

impl System for SDebugFps
{
    fn register(handlers: &mut Systems)
    {
        handlers.insert::<evt::Update>(0, Self::on_update);
    }
}

impl SDebugFps
{
    fn on_update(app: &mut Application)
    {
        println!("{} FPS", (1.0 / app.time().dt()).round());
    }
}