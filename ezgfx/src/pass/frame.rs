/// stores the output texture for a rendering frame,
/// from the swapchain. also contains the command encoder
/// when creating render passes.
pub struct Frame
{
    pub(crate) output: wgpu::SwapChainOutput,

    pub(crate) encoder: Option<wgpu::CommandEncoder>,
}

impl Frame
{
    /// create a new frame. this shouldn't be called
    /// directly.
    pub(crate) fn new(ctx: &mut crate::Renderer) -> Self
    {
        let output = ctx.sc
            .get_next_texture()
            .expect("timeout getting texture");

        Self { output, encoder: None }
    }
}