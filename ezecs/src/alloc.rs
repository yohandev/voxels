use crate::Entity;

/// entity allocator. keeps track of dead and living
/// entities.
#[derive(Debug, Default)]
pub struct EntAlloc
{
    entities: Vec<Entity>,
    free: Vec<usize>,
}

impl EntAlloc
{
    /// create a new entity allocator
    pub fn new() -> Self
    {
        Default::default()
    }

    /// allocate a new entity
    pub fn alloc(&mut self) -> Entity
    {
        if let Some(free) = self.free.pop()
        {
            self.entities[free].spawn();
            self.entities[free]
        }
        else
        {
            let id = self.entities.len();

            self.entities.push(Entity::new(id));
            self.entities[id]
        }
    }

    /// free an existing entity
    pub fn free(&mut self, ent: Entity)
    {
        let id = ent.id() as usize;

        self.entities[id].kill();
        self.free.push(id);
    }
}