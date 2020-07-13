use ezgame::ecs::*;
use ezgame::*;
use ezmath::*;

use crate::common::states::GameState;
use crate::common::CHUNK_SIZE;

/// chunk loading system
pub struct SChunkLoad;

impl System for SChunkLoad
{
    fn register(handlers: &mut Systems)
    {
        handlers.insert::<evt::StateChanged>(-999, Self::on_state_change);
    }
}

impl SChunkLoad
{
    fn on_state_change(app: &mut Application)
    {
        const SIZE: i32 = CHUNK_SIZE as i32;

        if let Some(state) = app.state().get_mut::<GameState>()
        {
            // temporary load on start
            (0..5)
            .flat_map(|x| (0..5).map(move |z| (x, z)))
            .flat_map(|(x, z)| (-2..2).map(move |y| (x, y, z)))
            .for_each(|(x, y, z)|
            {
                state.world.load_chunk(int3::new(x, y, z) * SIZE);
            });
        }
    }
}