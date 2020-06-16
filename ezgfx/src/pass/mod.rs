use crate::*;

pub struct RenderPass<'a>
{
    pass: Option<wgpu::RenderPass<'a>>,

    encoder: &'a mut wgpu::CommandEncoder,
    view: &'a wgpu::TextureView,

    // -- cache --
    current_indices_len: usize,
}

impl<'a> RenderPass<'a>
{
    /// create render pass. this shouldn't be called directly.
    pub(crate) fn new(encoder: &'a mut wgpu::CommandEncoder, view: &'a wgpu::TextureView) -> Self
    {
        Self
        {
            pass: None,

            encoder,
            view,

            current_indices_len: 0,
        }
    }

    /// begin the render pass and clear the output texture with the
    /// given colour
    pub fn begin_clear(&mut self, r: f64, g: f64, b: f64, a: f64)
    {
        self.encoder.begin_render_pass
        (
            &wgpu::RenderPassDescriptor
            {
                color_attachments:
                &[
                    wgpu::RenderPassColorAttachmentDescriptor
                    {
                        attachment: self.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color { r, g, b, a, }
                    }
                ],
                depth_stencil_attachment: None,
            }
        );
    }

    /// bind geometry for drawing
    pub fn geometry<V: Vertex, I: Index>(&mut self, geo: &'a Geometry<V, I>)
    {
        let pass = self.pass
            .as_mut()
            .expect("attempting to bind geometry before a RenderPass::begin_* call!");

        pass.set_vertex_buffer(0, &geo.v_buf, 0, 0);
        pass.set_index_buffer(&geo.i_buf, 0, 0);

        self.current_indices_len = geo.i_len;
    }

    /// set a bind group at a given set slot
    pub fn bind_group(&mut self, set: u32, group: &'a dyn IBindGroup)
    {
        let pass = self.pass
            .as_mut()
            .expect("attempting to bind a bind group before a RenderPass::begin_* call!");

        pass.set_bind_group(set, group.bind(), &[]);
    }

    /// draw the last set geometry
    pub fn draw(&mut self, instances: std::ops::Range<u32>)
    {
        let pass = self.pass
            .as_mut()
            .expect("attempting to draw geometry before a RenderPass::begin_* call!");

        pass.draw_indexed(0..self.current_indices_len as u32, 0, instances);
    }
}

// /// a render pass is a recorder for rendering instructions
// pub struct RenderPass<'a>
// {
//     ctx: &'a mut Renderer,

//     pass: wgpu::RenderPass<'a>,

//     geo_index_len: usize,
// }

// impl<'a> RenderPass<'a>
// {
//     /// create a new render pass. this should not be called
//     /// directly.
//     pub(crate) fn new(ctx: &'a mut Renderer, clear: [f64; 4]) -> Self
//     {
//         let pass = ctx.pass_cmds
//             .as_mut()
//             .expect("no command encoder! did you forget to call Renderer::frame()?")
//             .begin_render_pass
//         (
//             &wgpu::RenderPassDescriptor
//             {
//                 color_attachments:
//                 &[
//                     wgpu::RenderPassColorAttachmentDescriptor
//                     {
//                         attachment: &ctx.frame
//                             .as_ref()
//                             .expect("no render frame! did you forget to call Renderer::frame()?")
//                             .view,
//                         resolve_target: None,
//                         load_op: wgpu::LoadOp::Clear,
//                         store_op: wgpu::StoreOp::Store,
//                         clear_color: wgpu::Color
//                         {
//                             r: clear[0],
//                             g: clear[1],
//                             b: clear[2],
//                             a: clear[3],
//                         }
//                     }
//                 ],
//                 depth_stencil_attachment: None,
//             }
//         );

//         Self
//         {
//             ctx,
//             pass,
//             geo_index_len: 0,
//         }
//     }

//     /// bind geometry for drawing
//     pub fn geometry<V: Vertex, I: Index>(&mut self, geo: &'a Geometry<V, I>)
//     {
//         self.pass.set_vertex_buffer(0, &geo.v_buf, 0, 0);
//         self.pass.set_index_buffer(&geo.i_buf, 0, 0);

//         self.geo_index_len = geo.i_len;
//     }

//     /// set a bind group at a given set slot
//     pub fn bind_group(&mut self, set: u32, group: &'a wgpu::BindGroup)
//     {
//         self.pass.set_bind_group(set, group, &[]);
//     }

//     /// draw the last set geometry
//     pub fn draw(&mut self, instances: std::ops::Range<u32>)
//     {
//         self.pass.draw_indexed(0..self.geo_index_len as u32, 0, instances);
//     }

//     /// submit this render pass for rendering
//     pub fn submit(self)
//     {
//         self.ctx.queue.submit
//         (
//             &[self.ctx.pass_cmds.take().unwrap().finish()]
//         );
//     }
// }