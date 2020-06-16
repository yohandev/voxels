// use crate::*;

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