use super::*;

impl RenderCtx
{
    pub fn create_render_pass<'a>(&'a self, frame: &'a SwapChainOutput, encoder: &'a mut CommandEncoder) -> RenderPassBuilder
    {
        RenderPassBuilder
        {
            ctx: self,

            frame,
            encoder,

            load: LoadOp::Clear,
            store: StoreOp::Store,
            
            clear: double4::new(0.0, 0.0, 0.0, 1.0),
            depth: None
        }
    }
}

pub struct RenderPassBuilder<'a>
{
    ctx: &'a RenderCtx,

    frame: &'a SwapChainOutput,
    encoder: &'a mut CommandEncoder,

    load: LoadOp,
    store: StoreOp,

    clear: double4,
    depth: Option<RenderPassDepthStencilAttachmentDescriptor<'a>>,
}

impl<'a> RenderPassBuilder<'a>
{
    pub fn with_load(mut self, load: LoadOp) -> Self
    {
        self.load = load;
        self
    }

    pub fn with_store(mut self, store: StoreOp) -> Self
    {
        self.store = store;
        self
    }

    pub fn with_clear(mut self, clear: double4) -> Self
    {
        self.clear = clear;
        self
    }

    // TODO pub fn with_depth(mut self, ..)

    pub fn build(self) -> RenderPass<'a>
    {
        self.encoder.begin_render_pass
        (
            &RenderPassDescriptor
            {
                color_attachments:
                &[
                    RenderPassColorAttachmentDescriptor
                    {
                        attachment: &self.frame.view,
                        resolve_target: None,
                        load_op: self.load,
                        store_op: self.store,
                        clear_color: Color
                        {
                            r: self.clear.x,
                            g: self.clear.y,
                            b: self.clear.z,
                            a: self.clear.w
                        }
                    }
                ],
                depth_stencil_attachment: self.depth
            }
        )
    }
}