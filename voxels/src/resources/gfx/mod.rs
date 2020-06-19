mod simple;
mod chunk;

pub use simple::*;
pub use chunk::*;

/// add all the gfx resources to the application
pub fn add_gfx_resources(app: &mut ezgame::Application)
{
    app.resources().insert(SimpleGfxResources::None);
    app.resources().insert(ChunkGfxResources::None);
}