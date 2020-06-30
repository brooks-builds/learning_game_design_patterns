use ggez::event::winit_event::WindowEvent;
use ggez::event::{EventHandler, EventsLoop};
use ggez::input::mouse;
use ggez::timer;
use ggez::{Context, GameResult};
use std::time::Duration;

const FRAMES_PER_SECOND_TARGET: Duration = Duration::from_millis(1000 / 90);

pub fn run<S>(ctx: &mut Context, events_loop: &mut EventsLoop, state: &mut S) -> GameResult
where
    S: EventHandler,
{
    use ggez::input::keyboard;

    while ctx.continuing {
        let start_time = std::time::Instant::now();
        // If you are writing your own event loop, make sure
        // you include `timer_context.tick()` and
        // `ctx.process_event()` calls.  These update ggez's
        // internal state however necessary.
        ctx.timer_context.tick();
        events_loop.poll_events(|event| {
            ctx.process_event(&event);
            match event {
                ggez::event::winit_event::Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(logical_size) => {
                        // let actual_size = logical_size;
                        state.resize_event(
                            ctx,
                            logical_size.width as f32,
                            logical_size.height as f32,
                        );
                    }
                    WindowEvent::CloseRequested => {
                        if !state.quit_event(ctx) {
                            ggez::event::quit(ctx);
                        }
                    }
                    WindowEvent::Focused(gained) => {
                        state.focus_event(ctx, gained);
                    }
                    WindowEvent::ReceivedCharacter(ch) => {
                        state.text_input_event(ctx, ch);
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            ggez::event::winit_event::KeyboardInput {
                                state: ggez::event::winit_event::ElementState::Pressed,
                                virtual_keycode: Some(keycode),
                                modifiers,
                                ..
                            },
                        ..
                    } => {
                        let repeat = keyboard::is_key_repeated(ctx);
                        state.key_down_event(ctx, keycode, modifiers.into(), repeat);
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            ggez::event::winit_event::KeyboardInput {
                                state: ggez::event::winit_event::ElementState::Released,
                                virtual_keycode: Some(keycode),
                                modifiers,
                                ..
                            },
                        ..
                    } => {
                        state.key_up_event(ctx, keycode, modifiers.into());
                    }
                    WindowEvent::MouseWheel { delta, .. } => {
                        let (x, y) = match delta {
                            ggez::event::winit_event::MouseScrollDelta::LineDelta(x, y) => (x, y),
                            ggez::event::winit_event::MouseScrollDelta::PixelDelta(
                                winit::dpi::LogicalPosition { x, y },
                            ) => (x as f32, y as f32),
                        };
                        state.mouse_wheel_event(ctx, x, y);
                    }
                    WindowEvent::MouseInput {
                        state: element_state,
                        button,
                        ..
                    } => {
                        let position = mouse::position(ctx);
                        match element_state {
                            ggez::event::winit_event::ElementState::Pressed => {
                                state.mouse_button_down_event(ctx, button, position.x, position.y)
                            }
                            ggez::event::winit_event::ElementState::Released => {
                                state.mouse_button_up_event(ctx, button, position.x, position.y)
                            }
                        }
                    }
                    WindowEvent::CursorMoved { .. } => {
                        let position = mouse::position(ctx);
                        let delta = mouse::delta(ctx);
                        state.mouse_motion_event(ctx, position.x, position.y, delta.x, delta.y);
                    }
                    _x => {
                        // trace!("ignoring window event {:?}", x);
                    }
                },
                ggez::event::winit_event::Event::DeviceEvent { event, .. } => match event {
                    _ => (),
                },
                ggez::event::winit_event::Event::Awakened => (),
                ggez::event::winit_event::Event::Suspended(_) => (),
            }
        });
        state.update(ctx)?;
        state.draw(ctx)?;
        let tick_time_elapsed = start_time.elapsed();
        // if tick time elapsed is less than target draw FPS
        if tick_time_elapsed < FRAMES_PER_SECOND_TARGET {
            // then pause for difference between target and elapsed time
            let sleep_time = FRAMES_PER_SECOND_TARGET - tick_time_elapsed;
            timer::sleep(sleep_time);
        }
    }

    Ok(())
}
