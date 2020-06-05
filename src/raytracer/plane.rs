use crate::raytracer::*;

pub struct Plane
{
    pub center: float3,
    pub size: float2,

    pub material: Material
}

impl Renderable for Plane
{
    fn material(&self) -> &Material
    {
        &self.material
    }

    fn hits<'a>(&'a self, ray: &'a Ray) -> Option<RayCastHit<'a>>
    {
        if ray.direct.y.abs() < 1e-3
        {
            None
        }
        else
        {
            let d = -(ray.origin.y - self.center.y) / ray.direct.y;
            let pt = ray.origin + d * ray.direct;

            if d > 0.0
                && (pt.x - self.center.x).abs() < self.size.x
                && (pt.z - self.center.z).abs() < self.size.y
            {
                Some
                (
                    RayCastHit
                    {
                        distance: d,
                        point: pt,
                        normal: float3::new(0.0, 1.0, 0.0),
                        ray,
                        collide: self,
                        
                    }
                )   
            }
            else
            {
                None
            }
        }
    }
    
}