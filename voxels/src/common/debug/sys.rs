use ezgame::time::*;
use ezgame::ecs::*;

/// system that prints the frames per second
pub struct SDebugFps;

impl System for SDebugFps
{
    const EVENT: Event = evt::UPDATE;
    const ORDER: Order = ord::LOW;

    fn exe() -> SysFn
    {
        // begin...
        sys("debug_fps_system")
        // resources...
        .read_resource::<RTime>()
        // system...
        .build(|_, _, r_time, _|
        {
            println!("{} FPS", (1.0 / r_time.dt()).round());
        })
    }
}