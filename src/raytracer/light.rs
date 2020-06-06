use crate::raytracer::*;

pub trait Light: std::marker::Sync
{
    fn intensity(&self, hit: &RayCastHit, scene: &Scene) -> LightIntensity;
    fn goto(&mut self, pos: float3);
}

#[derive(Debug, Default)]
pub struct LightIntensity
{
    pub diffuse: f32,
    pub specular: f32,

    pub in_shadow: bool,
}

pub struct PointLight
{
    pub position: float3,
    pub intensity: f32
}

pub struct AmbientLight
{
    pub intensity: f32
}

impl Light for PointLight
{
    fn intensity(&self, hit: &RayCastHit, scene: &Scene) -> LightIntensity
    {
        let dis = (hit.point - self.position).magnitude();
        let dir = (hit.point - self.position) / dis;

        // -- shadow --
        let shadow_ray = Ray
        {
            origin: if dir.dot(&hit.normal) > 0.0
            {
                hit.point - (hit.normal * 1e-3)
            }
            else
            {
                hit.point + (hit.normal * 1e-3)
            },
            direct: -dir,
        };
        if let Some(shadow_hit) = shadow_ray.cast(scene)
        {
            if (shadow_hit.point - shadow_hit.ray.origin).magnitude_squared() < (dis * dis)
            {
                return LightIntensity { diffuse: 0.0, specular: 0.0, in_shadow: true }
            }
        }
        
        // -- diffuse --
        let diffuse = -self.intensity * max(0.0, dir.dot(&hit.normal));

        // -- specular --
        let specular = self.intensity * max(0.0, reflect(&dir, &hit.normal).dot(&hit.ray.direct)).powf(hit.collide.material().specular);
    
        LightIntensity { diffuse, specular, in_shadow: false }
    }

    fn goto(&mut self, pos: float3)
    {
        self.position = pos;
    }
}

impl Light for AmbientLight
{
    fn intensity(&self, _: &RayCastHit, _: &Scene) -> LightIntensity
    {
        LightIntensity { diffuse: self.intensity, specular: 0.0, in_shadow: false }
    }

    fn goto(&mut self, _: float3)
    {
        //self.position = pos;
    }
}