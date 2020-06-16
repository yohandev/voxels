use std::rc::Rc;

use super::Bind;

pub struct BindGroup<T: BindGroupTuple>
{
    pub bindings:   T,

    layout:         Rc<wgpu::BindGroupLayout>,
    bind:           wgpu::BindGroup,
}

pub trait BindGroupTuple { }

impl<T0: Bind> BindGroupTuple for T0
{

}

impl<T0: Bind, T1: Bind> BindGroupTuple for (T0, T1)
{

}

impl<T0: Bind, T1: Bind, T2: Bind> BindGroupTuple for (T0, T1, T2)
{

}