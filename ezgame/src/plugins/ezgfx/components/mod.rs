/// renderer component, representing an instance of an ezgfx
/// Renderer. the component is meaningless without the ezgame
/// provided Window component. this component is initially
/// uninitialized, and becomes init < 1 frame after the Window
/// component is succesfully initialized. use ```Renderer::is_init()```
/// to check whether this component is ready.
#[derive(Debug)]
pub struct Graphics(Option<ezgfx::Renderer>);

impl Graphics
{
    /// has this renderer been initialized using the window component
    /// on the same entity? if so, it should be safe to use the
    /// ezgfx::Renderer functionalities
    pub fn is_init(&self) -> bool
    {
        self.0.is_some()
    }

    /// initialize this component(internal)
    pub(crate) fn init(&mut self, window: &winit::window::Window)
    {
        let size = window.inner_size();

        self.0 = Some
        (
            ezgfx::Renderer::from_window(window, size.width, size.height)
        );
    }

    /// utility function
    fn unwrap(&self) -> &ezgfx::Renderer
    {
        self.0
            .as_ref()
            .expect(r"attempting to use renderer before it's initialized!
                consider using Renderer::is_init() to check before-hand!")
    }

    /// utility function
    fn unwrap_mut(&mut self) -> &mut ezgfx::Renderer
    {
        self.0
            .as_mut()
            .expect(r"attempting to use renderer before it's initialized!
                consider using Renderer::is_init() to check before-hand!")
    }

    /// create a new pipeline using the pipeline builder.
    /// the rendering pipeline is what takes your buffers:
    /// vertices, indices, uniforms, etc. and maps them to
    /// screen-space, rasterizes them, etc. The pipeline
    /// builder seeks to map part of that pipeline in an easy
    /// way. 
    pub fn pipeline(&self) -> ezgfx::PipelineBuilder
    {
        self.unwrap().pipeline()
    }

    /// create a brand new bind group, ensuing a new bind group layout
    /// and an actual bind group. see ezgfx::BindGroup for more.
    pub fn bind_group<T: ezgfx::BindGroupTuple>(&self, stage: ezgfx::ShaderKind, bindings: T) -> ezgfx::BindGroup<T>
    {
        self.unwrap().bind_group(stage, bindings)
    }

    /// create a new uniform buffer.
    pub fn uniform<T: ezgfx::BufferData>(&self, data: T) -> ezgfx::Uniform<T>
    {
        self.unwrap().uniform(data)
    }

    /// create new geometry. the slices passed in aren't consumed or
    /// stored to be retrieved later. you have to store them yourself
    /// to access them again, if needed.
    pub fn geometry<V: ezgfx::Vertex, I: ezgfx::Index>(&self, vertices: &[V], indices: &[I]) -> ezgfx::Geometry<V, I>
    {
        self.unwrap().geometry(vertices, indices)   
    }

    /// create a new shader module from its source code
    pub fn shader(&self, kind: ezgfx::ShaderKind, src: &str) -> ezgfx::Shader
    {
        self.unwrap().shader(kind, src)
    }

    /// get the next frame from the swapchain and begin a render pass,
    /// which encodes the rendering instructions and does the actual
    /// drawing.
    ///
    /// it takes in a rendering function, where all the rendering happens.
    pub fn render_pass<F>(&mut self, func: F) where F: for<'a> FnOnce(&'a mut ezgfx::Renderer, ezgfx::RenderPass<'a>)
    {
        self.unwrap_mut().render_pass(func);
    }
}

impl Default for Graphics
{
    fn default() -> Self
    {
        Self(None)
    }
}