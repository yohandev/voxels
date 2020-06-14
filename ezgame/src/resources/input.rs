use winit::event::{ VirtualKeyCode, MouseButton };

/// resource that stores raw keyboard, mouse, and [TODO] controller
/// input. It caches physical buttons that are held or up, as well as
/// buttons pressed or released during the duration of this frame.
pub struct Input
{
    pub(crate) cursor: [f64; 2],        // actual position
    pub(crate) scroll: [f32; 2],        // delta

    pub(crate) keys: [InputState; 255], // up, down, pressed, and released data
    pub(crate) btns: [InputState; 255], // same as keys, but for mouse buttons
}

/// enumeration to cache the state of input keys and buttons, used by
/// the Input resource and system. You can use these too through the
/// Input::key_state() and Input::button_state() functions, though
/// it's preffered to use the Input::key_down(), Input::key_pressed(),
/// etc.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum InputState
{
    /// input is not held for the duration of this frame
    Up = 0,
    /// input is held for the duration of this frame
    Down = 1,
    /// input started being pressed during this frame
    Pressed = 2,
    /// input stopped being pressed during this frame
    Released = 3,
}

impl Input
{
    pub(crate) fn new() -> Self
    {
        Input
        {
            cursor: [0.0; 2],
            scroll: [0.0; 2],

            keys: [InputState::Up; 255],
            btns: [InputState::Up; 255],
        }
    }

    /// is the key pressed this frame or held for the duration
    /// of this frame? useful for continuous input actions,
    /// such as character movement.
    pub fn key_down(&self, code: VirtualKeyCode) -> bool
    {
        match self.keys[code as usize]
        {
            InputState::Down => true,
            InputState::Pressed => true,
            _ => false
        }
    }

    /// was the key up and then pressed exactly during this frame?
    /// useful for impulsive actions, like jumping.
    pub fn key_pressed(&self, code: VirtualKeyCode) -> bool
    {
        self.keys[code as usize] == InputState::Pressed
    }

    /// was the key down and then released exactly druing this
    /// frame?
    pub fn key_released(&self, code: VirtualKeyCode) -> bool
    {
        self.keys[code as usize] == InputState::Released
    }

    /// opposite of Input::key_down()
    pub fn key_up(&self, code: VirtualKeyCode) -> bool
    {
        !self.key_down(code)
    }

    /// is the button pressed this frame or held for the duration
    /// of this frame? useful for continuous input actions,
    /// such as automatic weapons in a game.
    pub fn button_down(&self, code: MouseButton) -> bool
    {
        match self.btns[map_mouse_button(&code)]
        {
            InputState::Down => true,
            InputState::Pressed => true,
            _ => false
        }
    }

    /// was the button up and then pressed exactly during this frame?
    /// useful for impulsive actions, like shooting.
    pub fn button_pressed(&self, code: MouseButton) -> bool
    {
        self.btns[map_mouse_button(&code)] == InputState::Pressed
    }

    /// was the button down and then released exactly druing this
    /// frame?
    pub fn button_released(&self, code: MouseButton) -> bool
    {
        self.btns[map_mouse_button(&code)] == InputState::Released
    }

    /// opposite of Input::button_down()
    pub fn button_up(&self, code: MouseButton) -> bool
    {
        !self.button_down(code)
    }

    /// current mouse position, in window pixel coordinates
    pub fn cursor(&self) -> &[f64; 2]
    {
        &self.cursor
    }

    /// x position of the current mouse position, in window pixel
    /// coordinates.
    pub fn cursor_x(&self) -> f64
    {
        self.cursor[0]
    }

    /// y position of the current mouse position, in window pixel
    /// coordinates.
    pub fn cursor_y(&self) -> f64
    {
        self.cursor[1]
    }

    /// delta scroll during this frame. if you need the total scroll
    /// "position, " for some reason, you'll have to accumulate this
    /// value each frame.
    pub fn scroll(&self) -> &[f32; 2]
    {
        &self.scroll
    }

    /// delta scroll in the horizontal(x-axis) during this frame.
    pub fn scroll_x(&self) -> f32
    {
        self.scroll[0]
    }

    /// delta scroll in the vertical(y-axis) during this frame.
    pub fn scroll_y(&self) -> f32
    {
        self.scroll[1]
    }

    // /// time delta between this frame and the one before it
    // /// a frame is measured to be consecutive calls of the
    // /// event "app_update_event"
    // pub fn delta_time(&self) -> Duration
    // {
    //     self.delta
    // }

    // /// 32bit float representation of Input::delta_time()
    // pub fn delta_time_f32(&self) -> f32
    // {
    //     self.delta.as_secs_f32()
    // }

    /// get the state of the key for this frame. it's preferred
    /// to use Input::key_up(), Input::key_down(), etc.
    pub fn key_state(&self, code: VirtualKeyCode) -> InputState
    {
        self.keys[code as usize]
    }
}

/// utility function to map mouse buttons to a number
pub(crate) fn map_mouse_button(code: &MouseButton) -> usize
{
    match code
    {
        MouseButton::Left => 0,
        MouseButton::Right => 1,
        MouseButton::Middle => 2,
        MouseButton::Other(byte) => *byte as usize,
    }
}