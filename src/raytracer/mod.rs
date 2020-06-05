use crate::ezmath::*;

mod render;
mod camera;
mod sphere;
mod plane;
mod light;
mod scene;
mod ray;

pub use light::{ Light, LightIntensity, PointLight, AmbientLight };
pub use render::{ Renderable, Material };
pub use ray::{ Ray, RayCastHit };
pub use camera::Camera;
pub use sphere::Sphere;
pub use plane::Plane;
pub use scene::Scene;

// struct Sphere
// {
//     center: float3,
//     radius: f32,

//     mat: Material,
// }

// #[derive(Debug, Copy, Clone)]
// struct Material
// {
//     diffuse: [u8; 4],
// }

// struct Light
// {
//     position: float3,
//     intensity: f32,
// }

// struct RayCastResult
// {
//     mat: Material,
//     light: f32
// }

// trait RenderObject
// {
//     fn intersects(&self, ray: &Ray) -> (bool, f32);
//     fn material(&self) -> Material;
// }

// pub fn render(w: u32, h: u32, buf: &mut [u8])
// {
//     let scene = Scene
//     {
//         objects: &
//         [
//             Sphere
//             {
//                 center: float3::new(-3.0, 0.0, -16.0),
//                 radius: 2.0,
//                 mat: Material { diffuse: [100, 200, 50, 255] }
//             },
//             Sphere
//             {
//                 center: float3::new(3.0, 0.0, -15.0),
//                 radius: 1.0,
//                 mat: Material { diffuse: [0, 200, 0, 255] }
//             },
//             Sphere
//             {
//                 center: float3::new(7.0, 4.0, -13.0),
//                 radius: 3.0,
//                 mat: Material { diffuse: [250, 20, 60, 255] }
//             },
//             Sphere
//             {
//                 center: float3::new(-5.0, -3.0, -16.0),
//                 radius: 0.5,
//                 mat: Material { diffuse: [0, 255, 200, 255] }
//             },
//         ],
//         lights: &
//         [
//             Light
//             {
//                 position: float3::new(0.0, 10.0, -16.0),
//                 intensity: 0.7
//             }
//         ]
//     };

//     buf.par_chunks_mut(4).enumerate().for_each
//     (
//         |(i, pixel)|
//         {
//             let x = (i % w as usize) as f32;
//             let y = (i / w as usize) as f32;

//             let dx =  (2.0 * (x + 0.5) / (w as f32) - 1.0) * f32::tan(FOV / 2.0) * ((w as f32) / (h as f32));
//             let dy = -(2.0 * (y + 0.5) / (h as f32) - 1.0) * f32::tan(FOV / 2.0);
            
//             let ray = Ray
//             {
//                 origin: float3::new(0.0, 0.0, 0.0),
//                 direct: float3::new(dx, dy, -1.0).normalize(),
//             };
//             let res = cast_ray(&ray, &scene);
//             let out = res.mat.diffuse;

//             pixel[0] = clamp(out[0] as f32 * res.light, 0.0, 255.0) as u8;
//             pixel[1] = clamp(out[1] as f32 * res.light, 0.0, 255.0) as u8;
//             pixel[2] = clamp(out[2] as f32 * res.light, 0.0, 255.0) as u8;
//             pixel[3] =       out[3];
//         }
//     );
// }

// fn cast_ray<'a>(ray: &Ray, scene: &'a Scene<'a, Sphere>) -> RayCastResult
// {
//     let mut dist = f32::MAX;
//     let mut out = RayCastResult
//     {
//         mat: Material { diffuse: [50, 150, 200, 255] }, // background colour
//         light: 1.0
//     };

//     for obj in scene.objects
//     {
//         let intersect = obj.intersects(ray);
//         if intersect.0 && intersect.1 < dist
//         {
//             dist = intersect.1;
//             out.mat = obj.material();

//             // lighting
//             out.light = 0.0;
//             for light in scene.lights
//             {
//                 let dir = (light.position - obj.center).normalize();
                
//                 //out.light += light.intensity * dir
//             }
//         }
//     }
//     out
// }

// impl RenderObject for Sphere
// {
//     fn intersects(&self, ray: &Ray) -> (bool, f32)
//     {
//         let l = self.center - ray.origin;
//         let tca = l.dot(&ray.direct);
//         let d2 = l.dot(&l) - tca * tca;

//         if d2 > self.radius * self.radius
//         {
//             return (false, 0.0);
//         }

//         let thc = f32::sqrt(self.radius * self.radius - d2);
//         let mut t0  = tca - thc;
//         let t1 = tca + thc;
//         if t0 < 0.0
//         {
//             t0 = t1;
//         }
//         if t0 < 0.0
//         {
//             return (false, 0.0);
//         }

//         (true, t0)
//     }

//     fn material(&self) -> Material
//     {
//         self.mat
//     }
// }