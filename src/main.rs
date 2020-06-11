mod framework;
mod ezmath;
mod game;

use game::*;

fn main()
{
    Game::new()
        .with_graphics(800, 600)
        .run();
}