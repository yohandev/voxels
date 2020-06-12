use bytemuck::*;
use wgpu::*;

use crate::framework::RenderCtx;
use crate::ezmath::*;

pub mod pipelines;
pub mod uniforms;
pub mod vertices;

pub struct Gfx<'a>
{
    ctx: &'a RenderCtx
}

impl<'a> Gfx<'a>
{
    pub fn ctx(&self) -> &RenderCtx
    {
        self.ctx
    }
}