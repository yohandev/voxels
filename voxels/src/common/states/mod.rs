use std::rc::Rc;

use ezgame::ecs::*;
use ezgame::*;

use crate::common::block::RBlockPalette;
use crate::common::world::World;

/// the 'main' state where all the gaming 😎
/// happens
pub struct GameState
{
    /// overworld dimension
    pub world: World,
    /// living entity registry
    pub registry: Registry,
}

impl State for GameState
{
    fn create(app: &mut Application) -> Self where Self: Sized
    {
        Self
        {
            world: World::new(Rc::new(RBlockPalette::load(""))),
            registry: app.create_registry()
        }
    }

    fn registries(&self) -> &[&Registry]
    {
        &[&self.registry]
    }

    fn registries_mut(&self) -> &[&mut Registry]
    {
        &[&mut self.registry]
    }
}