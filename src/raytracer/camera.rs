use rayon::prelude::*;

use crate::raytracer::*;

pub struct Camera
{
    pub fov: f32,

    env_map: image::RgbImage
}

impl Default for Camera
{
    fn default() -> Self
    {        
        Camera
        {
            fov: 45.0 * (std::f32::consts::PI / 180.0),
            env_map: image::load_from_memory(include_bytes!("../old_outdoor_theater_2k.jpg"))
                .unwrap()
                .into_rgb()
        }
    }
}

impl Camera
{
    pub fn render(&self, scene: &Scene, width: u32, height: u32, frame: &mut [u8])
    {
        let wf32 = width as f32;
        let hf32 = height as f32;

        frame.par_chunks_mut(4).enumerate().for_each
        (
            |(i, pixel)|
            {
                let x = (i % width as usize) as f32;
                let y = (i / width as usize) as f32;

                let dx =  (2.0 * (x + 0.5) / wf32 - 1.0) * f32::tan(self.fov / 2.0) * (wf32 / hf32);
                let dy = -(2.0 * (y + 0.5) / hf32 - 1.0) * f32::tan(self.fov / 2.0);
                
                let ray = Ray
                {
                    origin: float3::new(0.0, 0.0, 0.0),
                    direct: float3::new(dx, dy, -1.0).normalize(),
                };

                pixel.copy_from_slice(&Self::calculate_pixel(self, &ray, scene, crate::BOUNCES));
            }
        );
    }

    pub fn calculate_pixel(&self, ray: &Ray, scene: &Scene, bounces: usize) -> [u8; 4]
    {
        if bounces <= 0
        {
            self.background(&ray.direct)
        }
        else if let Some(hit) = ray.cast(scene)              // hit object
        {
            // -- lighting --
            let light = scene.lights
                .iter()
                .map(|l| l.intensity(&hit, scene))
                .fold
                (
                    LightIntensity::default(),
                    |mut acc, i|
                    {
                        acc.diffuse += i.diffuse;
                        acc.specular += i.specular;
                        
                        acc
                    }
                );

            // -- reflection --
            let reflection = if hit.collide.material().reflectivity > 0.0
            {
                let reflect_dir = reflect(&hit.ray.direct, &hit.normal);
                let reflect_ray = Ray
                {
                    origin: if reflect_dir.dot(&hit.normal) < 0.0
                    {
                        hit.point - (hit.normal * 1e-3)
                    }
                    else
                    {
                        hit.point + (hit.normal * 1e-3)
                    },
                    direct: reflect_dir
                };
                scene.camera.calculate_pixel(&reflect_ray, scene, bounces - 1)
            }
            else
            {
                [0; 4]
            };

            // -- refraction --
            let refraction = if hit.collide.material().refractivity > 0.0
            {
                let refract_dir = refract(&hit.ray.direct, &hit.normal, hit.collide.material().ior).normalize();
                let refract_ray = Ray
                {
                    origin: if refract_dir.dot(&hit.normal) < 0.0
                    {
                        hit.point - (hit.normal * 1e-3)
                    }
                    else
                    {
                        hit.point + (hit.normal * 1e-3)
                    },
                    direct: refract_dir
                };
                scene.camera.calculate_pixel(&refract_ray, scene, bounces - 1)
            }
            else
            {
                [0; 4]
            };
            // if bounces == crate::BOUNCES
            // {
            //     return reflection;
            // }

            [
                Self::frag(&hit.collide.material(), 0, &light, &reflection, &refraction),
                Self::frag(&hit.collide.material(), 1, &light, &reflection, &refraction),
                Self::frag(&hit.collide.material(), 2, &light, &reflection, &refraction),
                255
            ]
        }
        else                                // background colour
        {
            self.background(&ray.direct)
        }
    }

    fn frag(mat: &Material, channel: usize, light: &LightIntensity, reflection: &[u8; 4], refraction: &[u8; 4]) -> u8
    {
        let diffuse = mat.diffuse[channel] as f32 * light.diffuse * mat.albedo[0];  
        let specular = light.specular * mat.albedo[1];
        let reflect = reflection[channel] as f32 * mat.reflectivity;
        let refract = refraction[channel] as f32 * mat.refractivity;

        clamp(diffuse + specular + reflect + refract, 0.0, 255.0) as u8
    }

    fn background(&self, dir: &float3) -> [u8; 4]
    {
        //-- sky gradient --
        // let v = (dir.y / 2.0) + 0.5;

        // let r = 44.0 + v * (90.0 - 44.0);
        // let g = 98.0 + v * (156.0 - 98.0);
        // let b = 145.0 + v * (214.0 - 145.0);

        // [r as u8, g as u8, b as u8, 255]

        // -- sphere map --
        // let m = 2.0 * ((dir.x * dir.x) + (dir.y * dir.y) + ((dir.z + 1.0) * (dir.z + 1.0))).sqrt();
        
        // let u = dir.x / m + 0.5;
        // let v = dir.y / m + 0.5;

        // let x = (u * self.env_map.width() as f32) as u32;
        // let y = (v * self.env_map.height() as f32) as u32;

        // let p = self.env_map.get_pixel(x, y);

        // [p.0[0], p.0[1], p.0[2], 255]

        // -- equirectangular map --
        let u = f32::atan2(dir.z, dir.x) * (std::f32::consts::FRAC_1_PI * 0.5) + 0.5;
        let v = f32::asin(-dir.y) * std::f32::consts::FRAC_1_PI + 0.5;

        let x = (u * self.env_map.width() as f32) as u32;
        let y = (v * self.env_map.height() as f32) as u32;

        let p = self.env_map.get_pixel(x, y);

        [p.0[0], p.0[1], p.0[2], 255]
    }
}