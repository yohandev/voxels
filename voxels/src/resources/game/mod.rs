mod chunk;

pub use chunk::*;

/// add all the game resources to the application
pub fn add_game_resources(app: &mut ezgame::Application)
{
    app.resources().insert(LoadedChunks::default());
}