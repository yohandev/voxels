use wgpu::*;

use crate::framework::RenderCtx;
use crate::ezmath::*;

mod chunk;

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