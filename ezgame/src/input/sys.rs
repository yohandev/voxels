use crate::ecs::*;
use crate::evt::*;
use super::*;

/// system that processes and caches input
/// events into the RInput resource.
pub struct SInput;

impl System<PollEvent> for SInput
{
    const ORDER: isize = 9999;

    fn run(&mut self, app: &mut crate::Application, evt: &PollEvent)
    {
        let mut r_input = app
            .resources()
            .get_mut_or_insert_with(|| RInput::new())
            .unwrap();

        if let winit::event::Event::NewEvents(_) = &evt.0
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
        if let winit::event::Event::WindowEvent { event, ..} = &evt.0
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