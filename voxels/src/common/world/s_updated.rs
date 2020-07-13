use ezgame::time::evt;
use ezgame::ecs::*;
use ezgame::*;

use crate::common::states::GameState;

/// system that 'un-sets' the updated chunk tag
/// on the very early update cycle(order = -999)
pub struct SChunkUpdated;

impl System for SChunkUpdated
{
    fn register(handlers: &mut Systems)
    {
        handlers.insert::<evt::Update>(-999, Self::on_update);
    }
}

impl SChunkUpdated
{
    fn on_update(app: &mut Application)
    {
        if let Some(state) = app.state().get_mut::<GameState>()
        {
            for chunk in state.world.chunks_mut()
            {
                chunk.unmark_updated();
            }
        }
    }
}