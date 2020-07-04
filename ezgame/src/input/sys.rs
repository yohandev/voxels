use crate::ecs::*;
use crate::evt;
use super::*;

/// system that processes and caches input
/// events into the RInput resource.
pub struct SInput;

impl System for SInput
{
    const EVENT: Event = evt::POLL;
    const ORDER: Order = ord::HIGH;
    
    fn prepare(r: &mut Resources)
    {
        r.insert(RInput::new());
    }

    fn exe() -> SysFn
    {
        // begin...
        sys("ezgame_input_system")
        // resources...
        .read_resource::<crate::window::REvent>()
        .write_resource::<RInput>()
        // system...
        .build(|_, _, (r_winit, r_input), _|
        {
            if let winit::event::Event::NewEvents(_) = &**r_winit
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
            if let winit::event::Event::WindowEvent { event, ..} = &**r_winit
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
        })
    }
}