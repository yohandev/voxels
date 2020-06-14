use ezgame::*;

fn main()
{ 
    Application::run::<TestGame>();
}

struct TestGame;

impl Game for TestGame
{
    fn build(app: &mut Application) -> Self
    {
        let world = app.create_world();

        {
            use components::*;

            world.insert((), vec![(Window::request(), WindowSize::default(), WindowTitle::default())]);
        }

        Self
    }
}