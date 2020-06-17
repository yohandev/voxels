pub mod gfx;

/// add the game's resources
pub fn add_resources(app: &mut ezgame::Application)
{
    gfx::add_gfx_resources(app);
}