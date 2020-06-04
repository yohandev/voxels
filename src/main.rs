mod raytracer;
mod window;
mod ezmath;

use window::*;

pub const WIDTH: u32 = 800;
pub const HEIGHT: u32 = 600;
pub const BOUNCES: usize = 3;

fn main()
{
    use raytracer::{ Sphere, Material, PointLight, AmbientLight };
    use ezmath::float3;

    let scene = raytracer::Scene
    {
        camera: Default::default(),
        objects: vec!
        [
            Box::new
            (
                Sphere
                {
                    center: float3::new(-3.0, 0.0, -16.0),
                    radius: 2.0,
                    material: Material
                    {
                        diffuse: [255, 255, 255],
                        albedo: [0.2, 10.0],
                        specular: 1425.0,
                        reflectivity: 0.8,
                    }
                }
            ),
            Box::new
            (
                Sphere
                {
                    center: float3::new(-1.0, -1.5, -12.0),
                    radius: 2.0,
                    material: Material
                    {
                        diffuse: [77, 26, 26],
                        albedo: [0.9, 0.1],
                        specular: 10.0,
                        reflectivity: 0.0,
                    }
                }
            ),
            Box::new
            (
                Sphere
                {
                    center: float3::new(1.5, -0.5, -18.0),
                    radius: 3.0,
                    material: Material
                    {
                        diffuse: [77, 26, 26],
                        albedo: [0.9, 0.1],
                        specular: 10.0,
                        reflectivity: 0.0,
                    }
                }
            ),
            Box::new
            (
                Sphere
                {
                    center: float3::new(7.0, 5.0, -18.0),
                    radius: 4.0,
                    material: Material
                    {
                        diffuse: [102, 102, 77],
                        albedo: [0.6, 0.3],
                        specular: 50.0,
                        reflectivity: 0.0,
                    }
                }
            ),
        ],
        lights: vec!
        [
            Box::new
            (
                PointLight
                {
                    position: float3::new(-20.0, 20.0, 20.0),
                    intensity: 1.5
                }
            ),
            Box::new
            (
                PointLight
                {
                    position: float3::new(30.0, 50.0, -25.0),
                    intensity: 1.8
                }
            ),
            Box::new
            (
                PointLight
                {
                    position: float3::new(30.0, 20.0, 30.0),
                    intensity: 1.7
                }
            ),
            Box::new
            (
                AmbientLight
                {
                    intensity: 0.1
                }
            )
        ],
        time: 0.0
    };

    Window::create("voxels", WIDTH, HEIGHT, Box::new(scene));
}