mod frame;

pub use frame::*;

use crate::*;

/// the render pass records rendering commands. create it
/// using ezgfx::Renderer::render_pass()
pub struct RenderPass<'a>
{
    pass: wgpu::RenderPass<'a>,

    // -- cache --
    current_indices_len: usize,
}

impl<'a> RenderPass<'a>
{
    /// create render pass. this shouldn't be called directly.
    pub(crate) fn new(frame: &'a mut Frame, clear: [f64; 4]) -> Self
    {
        let pass = frame.encoder
            .as_mut()
            .unwrap()
            .begin_render_pass
        (
            &wgpu::RenderPassDescriptor
            {
                color_attachments:
                &[
                    wgpu::RenderPassColorAttachmentDescriptor
                    {
                        attachment: &frame.output.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color
                        {
                            r: clear[0],
                            g: clear[1],
                            b: clear[2],
                            a: clear[3],
                        }
                    }
                ],
                depth_stencil_attachment: None,
            }
        );
        Self
        {
            pass,

            current_indices_len: 0,
        }
    }

    /// bind geometry for drawing
    pub fn geometry<V: Vertex, I: Index>(&mut self, geo: &'a Geometry<V, I>)
    {
        self.pass.set_vertex_buffer(0, &geo.v_buf, 0, 0);
        self.pass.set_index_buffer(&geo.i_buf, 0, 0);

        self.current_indices_len = geo.i_len;
    }

    /// set a bind group at a given set slot
    pub fn bind_group(&mut self, set: u32, group: &'a dyn IBindGroup)
    {
        self.pass.set_bind_group(set, group.bind(), &[]);
    }

    /// set the render pipeline to use for the next draw call
    pub fn pipeline(&mut self, pipeline: &'a Pipeline)
    {
        self.pass.set_pipeline(&pipeline.0);
    }

    /// draw the last set geometry
    pub fn draw(&mut self, instances: std::ops::Range<u32>)
    {
        self.pass.draw_indexed(0..self.current_indices_len as u32, 0, instances);
    }
}