use super::*;

pub trait Game
{
    fn build(app: &mut Application) -> Self;
}