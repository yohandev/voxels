use crate::raytracer::*;

pub struct Scene
{
    pub camera: Camera,

    pub objects: Vec<Box<dyn Renderable>>,
    pub lights: Vec<Box<dyn Light>>,

    pub time: f32,
}

impl crate::window::State for Scene
{
    fn render(&self, width: u32, height: u32, frame: &mut [u8])
    {
        self.camera.render(self, width, height, frame);
    }
    fn update(&mut self, dt:f32, _: &winit_input_helper::WinitInputHelper)
    {
        self.time += dt;

        self.lights[0].as_mut().goto(float3::new(self.time.cos() * 10.0, self.time.sin() * 10.0, -30.0));

        println!("delta time: {}", dt);
    }
    
}