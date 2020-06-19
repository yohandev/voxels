mod controller;
mod chunkgen;
mod loaded;

pub fn add_systems(app: &mut ezgame::Application)
{
    app.register_schedule
    (
        ezgame::events::APP_UPDATE,
        ezgame::legion::Schedule::builder()
            .add_system(controller::system())
            .add_system(loaded::system())
            .add_system(chunkgen::system())
            .build()
    );
}