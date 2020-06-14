/// an event polled directly from winit. you shouldn't need
/// to read(let alone write) to this resource at all.
/// instead, use the Input or DeltaTime for io needs.
pub struct WinitEvent(pub winit::event::Event<'static, ()>);