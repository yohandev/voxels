use crate::ecs::Systems;
use crate::Application;

pub trait Game
{
    /// build the game by registering its systems and bundles
    fn build(app: &mut Application, sys: &mut Systems) -> Self;
}