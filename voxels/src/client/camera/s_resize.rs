use ezgame::ecs::*;
use ezgame::window;

/// updates the camera matrix on window resize
pub struct SCameraResize;

impl System for SCameraResize
{
    const EVENT: Event = window::evt::RESIZED;
    const ORDER: Order = ord::MID;

    fn exe() -> Job
    {
        // begin...
        sys("camera_resize_system")
        // components...
        .with_query(<Write<super::CCamera>>::query())
        // resources...
        .read_resource::<window::RWindow>()
        // system
        .build(|_, world, window, query|
        {
            let size = window
                .as_ref()
                .unwrap()
                .inner_size();

            for mut cam in query.iter_mut(world)
            {
                cam.resize(size.width as f32, size.height as f32);
            }
        })
    }
}