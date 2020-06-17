mod gfx;

/// add the game's systems
pub fn add_systems(app: &mut ezgame::Application)
{
    gfx::add_gfx_systems(app);
}