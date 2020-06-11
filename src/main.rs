mod framework;
mod ezmath;
mod game;

use framework::*;
use game::*;

fn main()
{
    Application::create("voxels game")
        .with_graphics(800, 600)
        .run(Game {});   
}