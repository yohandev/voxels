use crate::resources::{ Input, InputState, WinitEvent, map_mouse_button };

use super::*;

/// system that processes and caches input events into the
/// ezgame::resources::Input resource.
pub fn input_system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("input_system")
        .read_resource::<WinitEvent>()
        .write_resource::<Input>()
        .build(|_, _, (event, input_res), _|
        {
            if let winit::event::Event::NewEvents(_) = &event.0
            {
                // reset keyboard keys
                for key in input_res.keys.iter_mut()
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
                for btn in input_res.btns.iter_mut()
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
                input_res.scroll[0] = 0.0;
                input_res.scroll[0] = 0.0;
            }
            if let winit::event::Event::WindowEvent { event, ..} = &event.0
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
                                    input_res.keys[code] = if input_res.keys[code] == InputState::Up
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
                                    input_res.keys[code] = if input_res.keys[code] == InputState::Down
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
                        input_res.cursor[0] = position.x;
                        input_res.cursor[1] = position.y;
                    }
                    winit::event::WindowEvent::MouseWheel { delta, .. } =>
                    {
                        const PIXELS_PER_LINE: f32 = 38.0;

                        match delta
                        {
                            winit::event::MouseScrollDelta::LineDelta(x, y) =>
                            {
                                input_res.scroll[0] += x;
                                input_res.scroll[1] += y;
                            }
                            winit::event::MouseScrollDelta::PixelDelta(dt) =>
                            {
                                input_res.scroll[0] += dt.x as f32 / PIXELS_PER_LINE;
                                input_res.scroll[1] += dt.y as f32 / PIXELS_PER_LINE;
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
                                input_res.keys[code] = if input_res.keys[code] == InputState::Up
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
                                input_res.keys[code] = if input_res.keys[code] == InputState::Down
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
        })
}