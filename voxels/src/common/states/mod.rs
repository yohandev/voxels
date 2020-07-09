use std::rc::Rc;

use crate::common::block::RBlockPalette;
use crate::common::world::World;

/// the 'main' state where all the gaming 😎
/// happens
pub struct GameState
{
    pub world: World,
}

impl Default for GameState
{
    fn default() -> Self
    {
        Self
        {
            world: World::new(Rc::new(RBlockPalette::load("")))
        }
    }
}