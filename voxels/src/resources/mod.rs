pub mod game;
pub mod gfx;

/// add the game's resources
pub fn add_resources(app: &mut ezgame::Application)
{
    game::add_game_resources(app);
    gfx::add_gfx_resources(app);
}