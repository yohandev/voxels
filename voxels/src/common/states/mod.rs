use std::rc::Rc;

use ezgame::ecs::*;
use ezgame::*;

use crate::common::block::RBlockPalette;
use crate::common::world::World;

/// the 'main' state where all the gaming 😎
/// happens
pub struct GameState
{
    pub world: World,
    pub registry: Registry,
}

impl State for GameState
{
    fn create() -> Self where Self: Sized
    {
        Self
        {
            world: World::new(Rc::new(RBlockPalette::load("")))
        }
    }

    fn registries(&self) -> &[&Registry]
    {
        todo!()
    }

    fn registries_mut(&self) -> &[&mut Registry]
    {
        todo!()
    }
}