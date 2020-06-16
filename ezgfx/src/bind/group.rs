use std::rc::Rc;

use crate::{ Bind, Renderer, ShaderKind };

/// a bind group is a collection of one or more shader resources,
/// such as textures, samplers, or uniforms. it can be bound to
/// a pipeline's set, to be used in shaders.
/// given the GLSL code:
/// ```glsl
/// layout(set=0, binding=0) uniform texture2D t_diffuse;
/// layout(set=0, binding=1) uniform sampler   s_diffuse;
///
/// layout(set=1, binding=0) uniform GlobalUniforms
/// {
///     mat4 u_view_projection;
/// };
/// ```
/// this example contains two bind groups, one at set = 0 and
/// at set = 1. The first bind group is composed of (Texture,
/// Sampler) while the other is (Uniform).
pub struct BindGroup<T: BindGroupTuple>
{
    /// get this bind group's bindings, which is a combination
    /// of shader resources. in GLSL, this could look like this:
    /// ```GLSL
    /// layout(set=0, binding=0) uniform texture2D t_diffuse;
    /// layout(set=0, binding=1) uniform sampler   s_diffuse;
    /// ```
    /// which maps to the (ezgfx::Texture, ezgfx::Sampler) tuple.
    pub bindings:   T,

    /// wgpu layout shared across bind groups
    layout:         Rc<wgpu::BindGroupLayout>,
    /// wgpu bind group unique to this object
    bind:           wgpu::BindGroup,
}

impl<T: BindGroupTuple> BindGroup<T>
{
    /// create a brand new bind group, ensuing a new wgpu::BindGroupLayout
    /// and wgpu::BindGroup. this should not be called directly.
    pub(crate) fn new(ctx: &Renderer, stage: ShaderKind, bindings: T) -> Self
    {
        let wgpu_stage = stage.to_wgpu();           // wgpu shader stage

        let layout = Rc::new                        // bind group layout
        (
            ctx.device.create_bind_group_layout
            (
                &wgpu::BindGroupLayoutDescriptor
                {
                    bindings: bindings
                        .layout_entries(wgpu_stage)
                        .as_slice(),
                    label: Some("ezgfx_bind_group_layout")
                }
            )
        );
        let bind = ctx.device.create_bind_group     // bind group
        (
            &wgpu::BindGroupDescriptor
            {
                layout: layout
                    .as_ref(),
                bindings: bindings
                    .bind_entries()
                    .as_slice(),
                label: Some("ezgfx_bind_group")
            }
        );

        Self { bindings, layout, bind }
    }
}

/// represents a variadic length tuple of bindings, to be used with
/// ezgfx::BindGroup. It combines different shader resources(which
/// implement the ezgfx::Bind trait, such as Uniform, Texture, Sampler,
/// etc.)
pub trait BindGroupTuple
{
    /// get the entries for the bind group layout. this is internal to
    /// ezgfx, shouldn't need to be called.
    fn layout_entries(&self, stage: wgpu::ShaderStage) -> Vec<wgpu::BindGroupLayoutEntry>;

    /// get the entries for the bind group. this is internal to ezgfx,
    /// shouldn't need to be called.
    fn bind_entries(&self) -> Vec<wgpu::Binding>;
}

impl<T0: Bind> BindGroupTuple for T0
{
    fn layout_entries(&self, stage: wgpu::ShaderStage) -> Vec<wgpu::BindGroupLayoutEntry>
    {
        vec!
        [
            wgpu::BindGroupLayoutEntry
            {
                binding: 0,
                visibility: stage,
                ty: self.binding_type(),
            },
        ]
    }

    fn bind_entries(&self) -> Vec<wgpu::Binding>
    {
        vec!
        [
            wgpu::Binding
            {
                binding: 0,
                resource: self.resource(),
            },
        ]
    }
}

impl<T0: Bind, T1: Bind> BindGroupTuple for (T0, T1)
{
    fn layout_entries(&self, stage: wgpu::ShaderStage) -> Vec<wgpu::BindGroupLayoutEntry>
    {
        vec!
        [
            wgpu::BindGroupLayoutEntry
            {
                binding: 0,
                visibility: stage,
                ty: self.0.binding_type(),
            },
            wgpu::BindGroupLayoutEntry
            {
                binding: 1,
                visibility: stage,
                ty: self.1.binding_type(),
            },
        ]
    }

    fn bind_entries(&self) -> Vec<wgpu::Binding>
    {
        vec!
        [
            wgpu::Binding
            {
                binding: 0,
                resource: self.0.resource(),
            },
            wgpu::Binding
            {
                binding: 1,
                resource: self.1.resource(),
            },
        ]
    }
}