use crate::raytracer::*;

pub struct Ray
{
    pub origin: float3,
    pub direct: float3,
}

pub struct RayCastHit<'a>
{
    pub distance: f32,
    pub point: float3,
    pub normal: float3,

    pub ray: &'a Ray,
    pub collide: &'a dyn Renderable,
}

impl Ray
{
    pub fn cast<'a>(&'a self, scene: &'a Scene) -> Option<RayCastHit<'a>>
    {
        let mut hit = None;
        let mut dis = f32::MAX;

        for obj in &scene.objects
        {
            if let Some(col) = obj.hits(self)
            {
                if col.distance < dis
                {
                    dis = col.distance;
                    hit = Some(col);
                }
            }
        }

        hit
    }
}