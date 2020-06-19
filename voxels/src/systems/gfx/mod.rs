mod camera;
mod render;
mod chunk;
mod init;

/// add all the gfx systems to the application
pub fn add_systems(app: &mut ezgame::Application)
{
    app.register_system(ezgame::plugins::winit::events::WINDOW_RESIZE, camera::system());
    app.register_system(ezgame::events::APP_RENDER, render::system());
    
    app.register_schedule
    (
        ezgame::plugins::ezgfx::events::EZGFX_READY,
        ezgame::legion::Schedule::builder()
            .add_system(init::system())
            .flush()
            .add_system(chunk::init::system())
            .build()
    );
    app.register_system(ezgame::events::APP_UPDATE, chunk::mesh::system());
}