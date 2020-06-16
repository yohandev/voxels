/// a struct that builds a bind group and its layout
/// in a render pipeline. see ezgfx::Renderer::pipeline()
/// for details.
pub struct BindGroupBuilder<'a>
{
    parent: ParentBuilder<'a>,

    resources: Vec<ShaderResource<'a>>,
    set: u32
}

/// Owning of parent pipeline-builder, only to return
/// it once this bind-group "scope" has been popped.
type ParentBuilder<'a> = crate::PipelineBuilder<'a>;

/// a union of a shader bindable resource, plus its
/// binding slot, and shader stage visibility.
struct ShaderResource<'a>
{
    bind: Box<&'a dyn crate::Bind>,
    slot: u32,
    stage: wgpu::ShaderStage
}

impl<'a> BindGroupBuilder<'a>
{
    /// create a new bind group builder. this shouldn't be
    /// called directly.
    pub(crate) fn new(ctx: ParentBuilder<'a>, slot: u32) -> Self
    {
        Self
        {
            parent: ctx,
            resources: vec![],
            set: slot,
        }
    }

    /// use an existing bind group and its layout as this
    /// bind group. this basically does all the work for the
    /// builder. ¯\_(ツ)_/¯
    ///
    /// this is useful for shader resources shared across shaders
    /// such as the model-view-projection uniform:
    /// ```rust
    /// let (mvp_bind, mvp_layout) = get_uniform_bind_group();
    /// let pipeline_a = renderer.pipeline()
    ///     .set(0)
    ///         .existing(&mvp_layout)
    ///     .build();
    /// let pipeline_b = renderer.pipeline()
    ///     .set(0)
    ///         .existing(&mvp_layout)
    ///     .set(1)
    ///         .binding(0, Vertex, Uniform::new("..."))
    ///         .build()
    ///     .build();
    /// ```
    /// now both pipeline a and b can use the same bind group
    /// resource. this method also "pops" the bind group scope
    /// just like BindGroupBuilder::build().
    pub fn existing(mut self, layout: &'a wgpu::BindGroupLayout) -> ParentBuilder<'a>
    {
        self.parent.sets[self.set as usize] = Some(layout);
        self.parent
    }

    /// set the shader resource at binding N, as in ```layout(set=A, binding=N)```.
    /// this method also creates a bind group layout in the process.
    pub fn binding(mut self, slot: u32, stage: crate::ShaderKind, res: &'a dyn crate::Bind) -> Self
    {
        self.resources.push
        (
            ShaderResource
            {
                bind: Box::new(res),
                slot,
                stage: stage.to_wgpu(),
            }
        );
        self
    }

    /// finalize this bind group and return the pipeline
    /// builder that created it. it takes a mutable borrow
    /// to an option tuple to populate with the bind group
    /// and its layout created by this method.  
    pub fn build(mut self, out: &'a mut Option<(wgpu::BindGroupLayout, wgpu::BindGroup)>) -> ParentBuilder<'a>
    {
        let mut layout_entries = Vec::with_capacity(self.resources.len());
        let mut bind_entries = Vec::with_capacity(self.resources.len());

        for res in self.resources
        {
            layout_entries.push(wgpu::BindGroupLayoutEntry
            {
                binding: res.slot,
                visibility: res.stage,
                ty: res.bind.binding_type(),
            });
            bind_entries.push(wgpu::Binding
            {
                binding: res.slot,
                resource: res.bind.resource(),
            });
        }

        let layout = self.parent.ctx.device.create_bind_group_layout
        (
            &wgpu::BindGroupLayoutDescriptor
            {
                bindings: layout_entries.as_slice(),
                label: None,
            }
        );
        let bind = self.parent.ctx.device.create_bind_group
        (
            &wgpu::BindGroupDescriptor
            {
                layout: &layout,
                bindings: bind_entries.as_slice(),
                label: None,
            }   
        );

        *out = Some((layout, bind));

        self.parent.sets[self.set as usize] = Some(&out.as_ref().unwrap().0);
        self.parent
    }
}