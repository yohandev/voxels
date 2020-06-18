mod ltw;

pub fn add_systems(app: &mut ezgame::Application)
{
    app.register_system(ezgame::events::APP_UPDATE, ltw::system());
}