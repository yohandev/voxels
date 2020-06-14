use crate::framework::*;
use crate::ezmath::*;
use crate::voxel::*;
use crate::gfx::*;
use crate::ecs::*;

pub struct Game
{
    ecs: Universe,

    worlds: Vec<Dimension>, // loaded worlds

    gfx: Option<Gfx>,       // graphics, loads on first render call
}

impl State for Game
{
    fn on_update(&mut self, input: &Input)
    {
        println!("updating @ {}ms of delta", input.delta_time_f32() * 1000.0);
        
        if let Some(gfx) = &mut self.gfx
        {
            if input.key_down(KeyCode::I)
            {
                gfx.cam_pos -= float3::new(0.0, 0.0, 1.0);
            }
            else if input.key_down(KeyCode::K)
            {
                gfx.cam_pos += float3::new(0.0, 0.0, 1.0);
            }
            if input.key_down(KeyCode::L)
            {
                gfx.cam_pos += float3::new(1.0, 0.0, 0.0);
            }
            else if input.key_down(KeyCode::J)
            {
                gfx.cam_pos -= float3::new(1.0, 0.0, 0.0);
            }
            if input.key_down(KeyCode::Space)
            {
                gfx.cam_pos += float3::new(0.0, 1.0, 0.0);
            }
            else if input.key_down(KeyCode::M)
            {
                gfx.cam_pos -= float3::new(0.0, 1.0, 0.0);
            }
        }
    }

    fn on_render(&mut self, window: &mut Window)
    {
        if let Some(gfx) = &self.gfx
        {
            if let Some(world) = self.worlds.first_mut()
            {
                if world.chunks().len() == 0
                {
                    for x in -2..2
                    {
                        for y in 0..2
                        {
                            for z in -2..2
                            {
                                let pos = CHUNK_SIZE as i32 * int3::new(x, y, z);
                                
                                world.load_chunk(pos);
                                world.remesh_chunk(pos, window.ctx(), gfx);
                            }
                        }
                    }
                }
                gfx.render(world, window);
            }
        }
        else
        {
            self.gfx = Some(Gfx::new(window.ctx()))
        }
    }
}

impl Game
{
    pub fn new() -> Self
    {
        let uni = Universe::new();

        Self
        {
            worlds: vec![Dimension::new(uni.create_world())],
            ecs: uni,
            gfx: None,
        }
    }

    pub fn _ecs(&self) -> &Universe
    {
        &self.ecs
    }
}