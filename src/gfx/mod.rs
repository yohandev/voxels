use bytemuck::*;
use wgpu::*;

use crate::framework::*;
use crate::ezmath::*;

pub mod pipelines;
pub mod uniforms;
pub mod vertices;

pub struct Gfx<'a>
{
    ctx: &'a RenderCtx,

    mvp: Uniform<uniforms::ModelViewProj>
}

impl<'a> Gfx<'a>
{
    pub fn ctx(&self) -> &RenderCtx
    {
        self.ctx
    }
}