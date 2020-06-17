mod simple;

pub use simple::*;

/// add all the gfx resources to the application
pub fn add_gfx_resources(app: &mut ezgame::Application)
{
    app.resources().insert(Option::<SimpleGfxResources>::None);
}