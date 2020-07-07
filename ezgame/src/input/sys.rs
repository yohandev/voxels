use crate::ecs::*;
use crate::*;
use super::*;

/// system that processes and caches input
/// events into the RInput resource.
pub struct SInput;

impl System for SInput
{
    fn register(handlers: &mut Systems)
    {
        handlers.insert::<crate::evt::Start>(-9999, Self::on_start);
        handlers.insert::<crate::evt::Poll>(-9999, Self::on_poll);
    }
}

impl SInput
{
    fn on_start(app: &mut Application)
    {
        app.resources().insert(RInput::new());
    }

    fn on_poll(app: &mut Application)
    {
        let (r_poll, mut r_input) = app.fetch_mut::<(Read<RWinitPoll>, Write<RInput>)>();

        if let winit::event::Event::NewEvents(_) = &*r_poll
        {
            // reset keyboard keys
            for key in r_input.keys.iter_mut()
            {
                *key = match *key
                {
                    InputState::Pressed => InputState::Down,
                    InputState::Released => InputState::Up,
                    InputState::Down => InputState::Down,
                    InputState::Up => InputState::Up,
                };
            }

            // reset mouse buttons
            for btn in r_input.btns.iter_mut()
            {
                *btn = match *btn
                {
                    InputState::Pressed => InputState::Down,
                    InputState::Released => InputState::Up,
                    InputState::Down => InputState::Down,
                    InputState::Up => InputState::Up,
                };
            }

            // reset mouse scroll
            r_input.scroll[0] = 0.0;
            r_input.scroll[1] = 0.0;

            // reset mouse delta
            r_input.delta[0] = 0.0;
            r_input.delta[1] = 0.0;
        }
        if let winit::event::Event::WindowEvent { event, ..} = &*r_poll
        {
            match event
            {
                winit::event::WindowEvent::KeyboardInput { input, .. } =>
                {
                    if let Some(code) = input.virtual_keycode
                    {
                        let code = code as usize;

                        match input.state
                        {
                            winit::event::ElementState::Pressed =>
                            {
                                r_input.keys[code] = if r_input.keys[code] == InputState::Up
                                {
                                    InputState::Pressed
                                }
                                else
                                {
                                    InputState::Down
                                };
                            }
                            winit::event::ElementState::Released =>
                            {
                                r_input.keys[code] = if r_input.keys[code] == InputState::Down
                                {
                                    InputState::Released
                                }
                                else
                                {
                                    InputState::Up
                                };
                            }
                        }
                    }
                }
                winit::event::WindowEvent::CursorMoved { position, .. } =>
                {
                    r_input.delta[0] = position.x - r_input.cursor[0];
                    r_input.delta[1] = position.y - r_input.cursor[1];

                    r_input.cursor[0] = position.x;
                    r_input.cursor[1] = position.y;
                }
                winit::event::WindowEvent::MouseWheel { delta, .. } =>
                {
                    const PIXELS_PER_LINE: f32 = 38.0;

                    match delta
                    {
                        winit::event::MouseScrollDelta::LineDelta(x, y) =>
                        {
                            r_input.scroll[0] += x;
                            r_input.scroll[1] += y;
                        }
                        winit::event::MouseScrollDelta::PixelDelta(dt) =>
                        {
                            r_input.scroll[0] += dt.x as f32 / PIXELS_PER_LINE;
                            r_input.scroll[1] += dt.y as f32 / PIXELS_PER_LINE;
                        }
                    }
                }
                winit::event::WindowEvent::MouseInput { state, button, .. } =>
                {
                    let code = map_mouse_button(&button);

                    match state
                    {
                        winit::event::ElementState::Pressed =>
                        {
                            r_input.btns[code] = if r_input.btns[code] == InputState::Up
                            {
                                InputState::Pressed
                            }
                            else
                            {
                                InputState::Down
                            };
                        }
                        winit::event::ElementState::Released =>
                        {
                            r_input.btns[code] = if r_input.btns[code] == InputState::Down
                            {
                                InputState::Released
                            }
                            else
                            {
                                InputState::Up 
                            };
                        }
                    }
                },
                _ => {}
            }
        }
    }
}