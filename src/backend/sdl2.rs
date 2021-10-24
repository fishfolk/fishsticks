pub use sdl2::controller::{Axis, Button};

use crate::analog::AnalogInputValue;
use crate::{Gamepad, GamepadId};
use std::collections::HashMap;

pub type ImplementationId = u32;

pub struct ImplementationGamepad(sdl2::controller::GameController);

impl super::BackendGamepad for ImplementationGamepad {
    fn connected(&self) -> bool {
        self.0.attached()
    }
}

pub struct ImplementationContext {
    sdl_context: sdl2::Sdl,
    controller_subsystem: sdl2::GameControllerSubsystem,
}

impl ImplementationContext {
    pub fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let controller_subsystem = sdl_context.game_controller()?;

        Ok(Self {
            sdl_context,
            controller_subsystem,
        })
    }
}

impl super::Backend for ImplementationContext {
    fn update(&mut self, gamepads: &mut HashMap<GamepadId, Gamepad>) -> Result<(), String> {
        let mut event_pump = self.sdl_context.event_pump()?;

        for (_, gamepad) in gamepads.iter_mut() {
            gamepad.update_inputs();
        }

        for event in event_pump.poll_iter() {
            use sdl2::event::Event;
            match event {
                Event::ControllerDeviceAdded { which, .. } => {
                    let gamepad = self.controller_subsystem.open(which);
                    if let Ok(gamepad) = gamepad {
                        #[cfg(debug_assertions)]
                        let name = gamepad.name();

                        gamepads.insert(
                            GamepadId(gamepad.instance_id()),
                            Gamepad::new(ImplementationGamepad(gamepad)),
                        );

                        #[cfg(debug_assertions)]
                        println!("Added gamepad \"{}\"", name);
                    }
                }
                Event::ControllerDeviceRemoved { which, .. } => {
                    #[cfg(debug_assertions)]
                    let name = gamepads
                        .get(&GamepadId(which))
                        .unwrap()
                        .internal_gamepad
                        .0
                        .name();

                    gamepads.remove(&GamepadId(which));

                    #[cfg(debug_assertions)]
                    println!("Removed gamepad \"{}\"", name);
                }
                Event::ControllerAxisMotion {
                    which, axis, value, ..
                } => {
                    if let Some(gamepad) = gamepads.get_mut(&GamepadId(which)) {
                        gamepad
                            .analog_inputs
                            .set(axis, AnalogInputValue::from(value));
                    }
                }
                Event::ControllerButtonDown { which, button, .. } => {
                    if let Some(gamepad) = gamepads.get_mut(&GamepadId(which)) {
                        gamepad.digital_inputs.activate(button);
                    }
                }
                Event::ControllerButtonUp { which, button, .. } => {
                    if let Some(gamepad) = gamepads.get_mut(&GamepadId(which)) {
                        gamepad.digital_inputs.deactivate(button);
                    }
                }
                _ => (),
            }
        }

        Ok(())
    }
}
