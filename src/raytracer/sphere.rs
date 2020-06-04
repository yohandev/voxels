use crate::raytracer::*;

pub struct Sphere
{
    pub center: float3,
    pub radius: f32,

    pub material: Material,
}

impl Renderable for Sphere
{
    fn material(&self) -> &Material
    {
        &self.material
    }

    fn hits<'a>(&'a self, ray: &'a Ray) -> Option<RayCastHit<'a>>
    {
        let l = self.center - ray.origin;
        let tca = l.dot(&ray.direct);
        let d2 = l.dot(&l) - tca * tca;

        if d2 > self.radius * self.radius
        {
            return None;
        }

        let thc = f32::sqrt(self.radius * self.radius - d2);
        let mut t0  = tca - thc;
        let t1 = tca + thc;
        if t0 < 0.0
        {
            t0 = t1;
        }
        if t0 < 0.0
        {
            return None;
        }

        let distance = t0;
        let point = ray.origin + (distance * ray.direct);
        let normal = (point - self.center).normalize();

        let out = Some(RayCastHit::<'a> { distance, point, normal, collide: self, ray });

        out
    }
}