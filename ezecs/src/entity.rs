/// type alias that represents the index of an entity,
/// which is basically it's ID
pub type EntInd = u32;
/// type alias that represents the generation of an
/// entity, to avoid comparing unmatching entities.
/// negative or zero values of this represent a dead
/// entity.
pub type EntGen = i64;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Entity
{
    ind: EntInd,
    gen: EntGen
}

impl Entity
{
    /// is this entity alive?
    pub fn alive(&self) -> bool
    {
        self.gen > 0
    }

    /// get the id or index of this entity
    pub fn id(&self) -> EntInd
    {
        self.ind
    }

    /// get the generation of this entity. it may
    /// return a negative number or zero, indicating
    /// that this entity is dead.
    pub fn gen(&self) -> EntGen
    {
        self.gen
    }

    /// alloc a new entity
    pub(crate) fn new(id: usize) -> Self
    {
        Self { ind: id as EntInd, gen: 0 }
    }

    /// mark this entity as dead, modifying its generation component
    pub(crate) fn kill(&mut self)
    {
        assert!(self.alive(), "cannot kill a dead entity!");

        self.gen *= -1; // make negative = dead
    }

    /// mark this entity as alive, modifying its generation component
    pub(crate) fn spawn(&mut self)
    {
        assert!(!self.alive(), "cannot spawn a living entity!");

        self.gen *= -1;
        self.gen += 1;
    }
}

use std::fmt::{ Display, Result };

impl Display for Entity
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result
    {
        if self.alive()
        {
            f.write_fmt(format_args!("entity(id: {}, gen: {})", self.ind, self.gen))
        }
        else
        {
            f.write_fmt(format_args!("dead entity(id: {}, last gen: {}", self.ind, self.gen))
        }
    }
}