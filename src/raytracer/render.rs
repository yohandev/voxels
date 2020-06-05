use crate::raytracer::*;

pub trait Renderable: std::marker::Sync
{
    fn material(&self) -> &Material;
    fn hits<'a>(&'a self, ray: &'a Ray) -> Option<RayCastHit<'a>>;
}

#[derive(Debug)]
pub struct Material
{
    pub diffuse: [u8; 3],
    pub albedo: [f32; 2],
    pub specular: f32,

    pub reflectivity: f32,
    pub refractivity: f32,
    pub ior: f32,
}