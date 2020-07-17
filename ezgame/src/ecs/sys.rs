/// an event-responding system, that operates on entities
/// and their components. it's convention that systems
/// are prefixed with an `S`. all the implementation of
/// the `Systems` trait does is the `register` method, where
/// you bind regular `impl` block methods to events.
///
/// # example
/// ```rust
/// struct SPhysics;
/// 
/// impl System for SPhysics
/// {
///     fn register(handlers: Systems)
///     {
///         handlers.insert::<evt::Start>(0, Self::on_start);
///         handlers.insert::<evt::Update>(-999, Self::on_update);
///         handlers.insert::<evt::Quit>(999, Self::on_quit);
///     }
/// }
///
/// impl System
/// {
///     // system functions must have this signature
///     fn on_start(_: Cmd, _: SubRegistry, ())
///     {
///         app.resources().insert(RGravity(-9.8));
///     }
///     
///     // you can call these methods whatever, but `on_{event}` is
///     // good practice
///     fn on_update(app: &mut Application)
///     {
///         if let Some(stage) = app.stage::<GameStage>()
///         {
///             let dt = app.time().dt();
///             let g  = app.res().get::<RGravity>();
///
///             let q = <Write<CVel>>::query();
///             
///             for mut vel in q.iter_mut(&mut stage.registry)
///             {
///                 vel += g * dt;
///             }
///             
///             let q = <(Write<CPos>, Read<CVel>)>::query();
///             
///             for (mut pos, vel) in q.iter_mut(&mut stage.registry)
///             {
///                 pos += vel * dt;
///             }
///         }
///     }
///     
///     // event handlers also don't need to be inside an `impl` block,
///     // but it's good for scoping
///     fn on_quit(app: &mut Application)
///     {
///         app.resources().remove::<RGravity>();
///     }
/// }
/// ```
pub trait System: 'static
{
    fn register(handlers: &mut Systems);
}