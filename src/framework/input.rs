pub mod KeyCode             // re-export key codes
{
    pub use winit::event::VirtualKeyCode::*;
}
use std::time::*;

use super::*;

pub struct Input
{
    cursor: double2,        // actual position
    scroll: float2,         // delta

    keys: [u8; 255],        // up, down, pressed, and released data
    btns: [u8; 255],        // same as keys, but for mouse buttons

    frame: Instant,         // last frame instant
    delta: Duration,        // delta duration between frame and the one before it
}

const ACT_UP:       u8 = 0; // not held for the duration of this frame
const ACT_PRESSED:  u8 = 1; // pressed this frame
const ACT_DOWN:     u8 = 2; // held for the duration of this frame
const ACT_RELEASED: u8 = 3; // released this frame

impl Input
{
    pub fn new() -> Self
    {
        Input
        {
            cursor: double2::new(0.0, 0.0),
            scroll: float2::new(0.0, 0.0),

            keys: [0; 255],
            btns: [0; 255],

            frame: Instant::now(),
            delta: Duration::default(),
        }
    }

    pub(super) fn process_events<T: State>(&mut self, event: &Event<()>, _: &mut ControlFlow, state: &mut T)
    {
        if let Event::NewEvents(_) = event
        {
            let now = Instant::now();

            self.delta = now.duration_since(self.frame);
            self.frame = now;

            for key in self.keys.iter_mut()
            {
                *key = match *key
                {
                    ACT_PRESSED => ACT_DOWN,
                    ACT_DOWN => ACT_DOWN,
                    ACT_UP => ACT_UP,
                    ACT_RELEASED => ACT_UP,
                    _ => 0
                };
            }
            for btn in self.btns.iter_mut()
            {
                *btn = match *btn
                {
                    ACT_PRESSED => ACT_DOWN,
                    ACT_DOWN => ACT_DOWN,
                    ACT_UP => ACT_UP,
                    ACT_RELEASED => ACT_UP,
                    _ => 0
                };
            }

            self.scroll.x = 0.0;
            self.scroll.y = 0.0;
        }
        if let Event::WindowEvent { event, ..} = event
        {
            match event
            {
                WindowEvent::KeyboardInput { input, .. } =>
                {
                    if let Some(code) = input.virtual_keycode
                    {
                        let code = code as usize;

                        match input.state
                        {
                            ElementState::Pressed =>
                            {
                                self.keys[code] = if self.keys[code] == ACT_UP { ACT_PRESSED } else { ACT_DOWN };
                            }
                            ElementState::Released =>
                            {
                                self.keys[code] = if self.keys[code] == ACT_DOWN { ACT_RELEASED } else { ACT_UP };
                            }
                        }
                    }
                }
                WindowEvent::CursorMoved { position, .. } =>
                {
                    self.cursor.x = position.x;
                    self.cursor.y = position.y;
                }
                WindowEvent::MouseWheel { delta, .. } =>
                {
                    const PIXELS_PER_LINE: f32 = 38.0;

                    match delta
                    {
                        MouseScrollDelta::LineDelta(x, y) =>
                        {
                            self.scroll.x += *x;
                            self.scroll.y += *y;
                        }
                        MouseScrollDelta::PixelDelta(dt) =>
                        {
                            self.scroll.x += dt.x as f32 / PIXELS_PER_LINE;
                            self.scroll.y += dt.y as f32 / PIXELS_PER_LINE;
                        }
                    }
                }
                WindowEvent::MouseInput { state, button, .. } =>
                {
                    let code = match button
                    {
                        MouseButton::Left => 0,
                        MouseButton::Right => 1,
                        MouseButton::Middle => 2,
                        MouseButton::Other(byte) => *byte as usize,
                    };

                    match state
                    {
                        ElementState::Pressed =>
                        {
                            self.keys[code] = if self.keys[code] == ACT_UP { ACT_PRESSED } else { ACT_DOWN };
                        }
                        ElementState::Released =>
                        {
                            self.keys[code] = if self.keys[code] == ACT_DOWN { ACT_RELEASED } else { ACT_UP };
                        }
                    }
                },
                _ => {}
            }
        }
        if let Event::MainEventsCleared = event
        {
            state.on_update(self);
        }
    }

    /// is the key pressed this frame or held for the duration of this frame?
    pub fn key_down(&self, code: VirtualKeyCode) -> bool
    {
        match self.keys[code as usize]
        {
            ACT_DOWN => true,
            ACT_PRESSED => true,
            _ => false
        }
    }

    /// was the key pressed this frame?
    pub fn key_pressed(&self, code: VirtualKeyCode) -> bool
    {
        self.keys[code as usize] == ACT_PRESSED
    }

    /// was the key released this frame?
    pub fn key_released(&self, code: VirtualKeyCode) -> bool
    {
        self.keys[code as usize] == ACT_RELEASED
    }

    /// opposite of key_down
    pub fn key_up(&self, code: VirtualKeyCode) -> bool
    {
        !self.key_down(code)
    }

    pub fn delta_time(&self) -> Duration
    {
        self.delta
    }

    pub fn delta_time_f32(&self) -> f32
    {
        self.delta.as_secs_f32()
    }
}