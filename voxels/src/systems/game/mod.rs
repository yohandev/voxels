mod controller;

pub fn add_systems(app: &mut ezgame::Application)
{
    app.register_system(ezgame::events::APP_UPDATE, controller::system())
}