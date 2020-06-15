mod builder;

pub struct Pipeline
{
    binds: Vec<crate::BindGroup>   
}

impl Pipeline
{
    /// get a resource within this pipeline at the set and
    /// binding. this mimic's GLSL uniform layout.
    pub fn resource<'a>(set: usize, binding: usize)// -> &'a Box<dyn Bind>
    {

    }
}
