use ezgame::legion::*;
use ezgame::*;

pub fn system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("debug_fps_system")
        .read_resource::<resources::Time>()
        .build(|_, _, time, _|
        {
            println!("dt: {}ms", time.delta_time_f32() * 1000.0);
        })
}