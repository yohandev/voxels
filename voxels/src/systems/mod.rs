mod transform;
mod debug;
mod game;
mod gfx;

/// add the game's systems
pub fn add_systems(app: &mut ezgame::Application)
{
    transform::add_systems(app);
    debug::add_systems(app);
    game::add_systems(app);
    gfx::add_systems(app);
}