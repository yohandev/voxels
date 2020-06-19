use std::sync::Arc;

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
#[derive(Debug)]
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
    layout:         Arc<wgpu::BindGroupLayout>,
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

        let layout = Arc::new                       // bind group layout
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

    /// create a new bind group "inspired" by this one, sharing
    /// the layout and pipeline compatibilities
    pub(crate) fn clone(&self, ctx: &Renderer, bindings: T) -> Self
    {
        let bind = ctx.device.create_bind_group     // bind group
        (
            &wgpu::BindGroupDescriptor
            {
                layout: self.layout
                    .as_ref(),
                bindings: bindings
                    .bind_entries()
                    .as_slice(),
                label: Some("ezgfx_bind_group_cloned")
            }
        );
        Self { bindings, layout: self.layout.clone(), bind }
    }
}

/// non-generic trait for ezgfx::BindGroup. Used to object-ify the otherwise
/// generic struct, to be used internally by ezgfx.
pub trait IBindGroup
{
    /// get this bind group's wgpu layout
    fn layout(&self) -> &wgpu::BindGroupLayout;

    /// get this bind group's wgpu bind
    fn bind(&self) -> &wgpu::BindGroup;
}

impl<T: BindGroupTuple> IBindGroup for BindGroup<T>
{
    fn layout(&self) -> &wgpu::BindGroupLayout
    {
        self.layout.as_ref()
    }

    fn bind(&self) -> &wgpu::BindGroup
    {
        &self.bind
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

/// macro that implements the ezgfx::BindGroupTuple trait for variadic
/// sized tuples.
macro_rules! impl_bind_group_tuple
{
    ({$($gen_name:ident),*}, {$($num:tt),*}) =>
    {
        impl<$($gen_name: Bind),*> BindGroupTuple for ($($gen_name),*,)
        {
            fn layout_entries(&self, stage: wgpu::ShaderStage) -> Vec<wgpu::BindGroupLayoutEntry>
            {
                vec!
                [
                    $(wgpu::BindGroupLayoutEntry
                    {
                        binding: $num,
                        visibility: stage,
                        ty: self.$num.binding_type(),
                    }),*
                ]
            }

            fn bind_entries(&self) -> Vec<wgpu::Binding>
            {
                vec!
                [
                    $(wgpu::Binding
                    {
                        binding: $num,
                        resource: self.$num.resource(),
                    }),*
                ]
            }
        }
    };
}

impl_bind_group_tuple!({T0}, {0});
impl_bind_group_tuple!({T0, T1}, {0, 1});
impl_bind_group_tuple!({T0, T1, T2}, {0, 1, 2});
impl_bind_group_tuple!({T0, T1, T2, T3}, {0, 1, 2, 3});
impl_bind_group_tuple!({T0, T1, T2, T3, T4}, {0, 1, 2, 3, 4});
impl_bind_group_tuple!({T0, T1, T2, T3, T4, T5}, {0, 1, 2, 3, 4, 5});
impl_bind_group_tuple!({T0, T1, T2, T3, T4, T5, T6}, {0, 1, 2, 3, 4, 5, 6});
impl_bind_group_tuple!({T0, T1, T2, T3, T4, T5, T6, T7}, {0, 1, 2, 3, 4, 5, 6, 7});
impl_bind_group_tuple!({T0, T1, T2, T3, T4, T5, T6, T7, T8}, {0, 1, 2, 3, 4, 5, 6, 7, 8});
impl_bind_group_tuple!({T0, T1, T2, T3, T4, T5, T6, T7, T8, T9}, {0, 1, 2, 3, 4, 5, 6, 7, 8, 9});
impl_bind_group_tuple!({T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10}, {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10});
impl_bind_group_tuple!({T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11}, {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11});
impl_bind_group_tuple!({T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12}, {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12});
impl_bind_group_tuple!({T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13}, {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13});
impl_bind_group_tuple!({T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14}, {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14});
impl_bind_group_tuple!({T0, T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15}, {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15});