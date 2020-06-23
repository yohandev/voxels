use crate::*;

/// a system is logic that acts on entities' components
pub trait System
{
    /// build the system, inserting required resources
    /// and returning the system closure. that closure
    /// is run everytime an event is polled.
    fn build(&self, app: &mut Application) -> SystemFn;
}

/// type alias for a legion system closure
pub type SystemFn = Box<dyn legion::Schedulable>;

/// a collection of systems, which really is just a
/// modular way of doing:
/// ```rust
/// add_window_systems(&mut app);
/// add_render_systems(&mut app);
/// add_health_systems(&mut app, 20.0);
/// ```
/// becoming:
/// ```rust
/// app.systems().insert(WindowBundle);
/// app.systems().insert(RenderBundle);
/// app.systems().insert(HealthBundle { max: 20.0 });
/// ```
pub trait SystemBundle
{
    /// add this bundle's systems to the application
    fn build(&self, app: &mut Application);
}

/// collection of systems merged into a schedule.
pub struct Systems
{
    exe: Option<legion::Schedule>,
    add: Option<ScheduleBuilder>,
}

/// alias for legion's module-hidden schedule builder
type ScheduleBuilder = ::legion::systems::schedule::Builder;

impl Systems
{
    pub(crate) fn new() -> Self
    {
        Self
        {
            exe: None,
            add: Some(Default::default()),
        }
    }

    /// insert a system into the schedule
    pub(crate) fn insert<T: System>(&mut self, app: &mut Application, sys: T)
    {
        assert!(self.add.is_some(), "systems already finalized!");

        self.add = self.add
            .take()
            .unwrap()
            .add_system(sys.build(app))
            .into();
    }

    /// build the schedule. systems can't be added afterwards
    pub(crate) fn finalize(&mut self)
    {
        assert!(self.add.is_some(), "systems already finalized!");

        self.exe = self.add
            .take()
            .unwrap()
            .build()
            .into();
    }

    /// execute the schedule
    pub(crate) fn execute(&mut self, worlds: &mut WorldList, res: &mut legion::Resources)
    {
        assert!(self.exe.is_some(), "systems can't be executed until finalized!");

        for world in worlds
        {
            self.exe
                .as_mut()
                .unwrap()
                .execute(world, res);
        }
    }
}