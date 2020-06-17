mod debug;
mod gfx;

/// add the game's systems
pub fn add_systems(app: &mut ezgame::Application)
{
    debug::add_systems(app);
    gfx::add_systems(app);
}