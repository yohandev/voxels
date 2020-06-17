mod render;
mod init;

/// add all the gfx systems to the application
pub fn add_gfx_systems(app: &mut ezgame::Application)
{
    app.register_system(ezgame::plugins::ezgfx::events::EZGFX_READY, init::system());
    app.register_system(ezgame::events::APP_RENDER, render::system());
}