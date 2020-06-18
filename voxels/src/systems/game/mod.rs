mod controller;
mod chunkgen;

pub fn add_systems(app: &mut ezgame::Application)
{
    app.register_system(ezgame::events::APP_UPDATE, controller::system());
    app.register_system(ezgame::events::APP_UPDATE, chunkgen::system());
}