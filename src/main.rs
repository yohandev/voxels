mod framework;
mod ezmath;
mod voxel;
mod game;
mod gfx;
mod ecs;

use framework::*;
use game::*;

fn main()
{
    Application::create("voxels game")
        .with_graphics(800, 600)
        .run(Game::new());   
}