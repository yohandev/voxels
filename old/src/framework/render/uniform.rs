use super::*;

impl RenderCtx
{
    pub fn create_uniform(&self) -> UniformBuilder
    {
        UniformBuilder
        {
            ctx: self,
            binding: 0,
            visibility: ShaderStage::NONE,
            label: None,
        }
    }
}

pub struct Uniform<T: Pod>
{
    buffer: Buffer,

    layout: BindGroupLayout,
    bind: BindGroup,

    ty: std::marker::PhantomData<T>
}

pub struct UniformBuilder<'a>
{
    ctx: &'a RenderCtx,

    binding: u32,
    visibility: ShaderStage,

    label: Option<&'a str>,
}

impl<T: Pod> Uniform<T>
{
    pub fn bind_group_layout(&self) -> &BindGroupLayout
    {
        &self.layout
    }

    pub fn bind_group(&self) -> &BindGroup
    {
        &self.bind
    }

    pub fn update_data(&self, ctx: &RenderCtx, data: T)
    {
        let mut encoder = ctx.create_command_encoder("uniform_update_encoder");
        let staging = ctx.create_buffer(&[data], BufferUsage::COPY_SRC);

        encoder.copy_buffer_to_buffer(&staging, 0, &self.buffer, 0, std::mem::size_of::<T>() as BufferAddress);
    
        ctx.submit(encoder);
    }
}

impl<'a> UniformBuilder<'a>
{
    pub fn with_binding_slot(mut self, slot: u32) -> Self
    {
        self.binding = slot;
        self
    }

    pub fn with_visiblity(mut self, stage: ShaderStage) -> Self
    {
        self.visibility = stage;
        self
    }

    pub fn with_label(mut self, label: &'a str) -> Self
    {
        self.label = Some(label);
        self
    }

    pub fn build<T: Pod>(self, data: T) -> Uniform<T>
    {
        let mut layout_label = self.label.unwrap_or("unnamed").to_string();
        let mut bind_label = layout_label.clone();

        layout_label.push_str("_bind_group_layout");
        bind_label.push_str("_bind_group");
        
        let buffer = self.ctx.create_buffer(&[data], BufferUsage::UNIFORM | BufferUsage::COPY_DST);
        let layout = self.ctx.device().create_bind_group_layout
        (
            &BindGroupLayoutDescriptor
            {
                bindings: &
                [
                    BindGroupLayoutEntry
                    {
                        binding: self.binding,
                        visibility: self.visibility,
                        ty: BindingType::UniformBuffer { dynamic: false },
                    }
                ],
                label: Some(layout_label.as_str()),
            }
        );
        let bind = self.ctx.device().create_bind_group
        (
            &BindGroupDescriptor
            {
                layout: &layout,
                bindings:
                &[
                    Binding
                    {
                        binding: self.binding,
                        resource: BindingResource::Buffer
                        {
                            buffer: &buffer,
                            range: 0..std::mem::size_of::<T>() as BufferAddress
                        }
                    }
                ],
                label: Some(bind_label.as_str())
            }
        );

        Uniform
        {
            buffer,
            layout: layout,
            bind,
            ty: std::marker::PhantomData::<T>{},
        }
    }
}