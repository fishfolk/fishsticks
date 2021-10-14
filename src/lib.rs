//! System for handling gamepad input.
//!
//! API is unstable and may change at any time.
//! Uses SDL2 as the backend.

#![warn(missing_docs)]

pub mod analog;
pub mod digital;

pub use sdl2::controller::{Axis, Button};

use analog::{AnalogInput, AnalogInputValue};
use digital::DigitalInput;
use std::collections::HashMap;

/// The instance Id of a gamepad.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GamepadId(u32);

/// Holds the state of a gamepad.
pub struct Gamepad {
    controller: sdl2::controller::GameController,
    /// Analog inputs, such as thumbsticks.
    pub analog_inputs: AnalogInput<Axis>,
    /// Digital inputs, such as buttons.
    pub digital_inputs: DigitalInput<Button>,
}

impl Gamepad {
    fn new(controller: sdl2::controller::GameController) -> Self {
        Self {
            controller,
            analog_inputs: Default::default(),
            digital_inputs: Default::default(),
        }
    }

    fn update_inputs(&mut self) {
        self.analog_inputs.update();
        self.digital_inputs.update();
    }
}

/// The gamepad system context.
///
/// Only one `GamepadContext` should be alive at any time.
pub struct GamepadContext {
    sdl_context: sdl2::Sdl,
    controller_subsystem: sdl2::GameControllerSubsystem,
    gamepads: HashMap<GamepadId, Gamepad>,
}

impl GamepadContext {
    /// Initializes the gamepad context.
    pub fn init() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let controller_subsystem = sdl_context.game_controller()?;
        let gamepads = HashMap::new();

        Ok(Self {
            sdl_context,
            controller_subsystem,
            gamepads,
        })
    }

    /// Gets a reference to a specific gamepad.
    ///
    /// Returns `None` if the gamepad is not found.
    pub fn gamepad(&self, id: GamepadId) -> Option<&Gamepad> {
        self.gamepads.get(&id)
    }

    /// Gets an iterator over all connected gamepads.
    pub fn gamepads(&self) -> impl Iterator<Item = (GamepadId, &Gamepad)> {
        self.gamepads
            .iter()
            .filter(|(_, gamepad)| gamepad.controller.attached())
            .map(|(&id, gamepad)| (id, gamepad))
    }

    /// Updates the state of all gamepads.
    pub fn update(&mut self) -> Result<(), String> {
        let mut event_pump = self.sdl_context.event_pump()?;

        for (_, gamepad) in self.gamepads.iter_mut() {
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

                        self.gamepads
                            .insert(GamepadId(gamepad.instance_id()), Gamepad::new(gamepad));

                        #[cfg(debug_assertions)]
                        println!("Added gamepad \"{}\"", name);
                    }
                }
                Event::ControllerDeviceRemoved { which, .. } => {
                    #[cfg(debug_assertions)]
                    let name = self
                        .gamepads
                        .get(&GamepadId(which))
                        .unwrap()
                        .controller
                        .name();

                    self.gamepads.remove(&GamepadId(which));

                    #[cfg(debug_assertions)]
                    println!("Removed gamepad \"{}\"", name);
                }
                Event::ControllerAxisMotion {
                    which, axis, value, ..
                } => {
                    if let Some(gamepad) = self.gamepads.get_mut(&GamepadId(which)) {
                        gamepad
                            .analog_inputs
                            .set(axis, AnalogInputValue::from(value));
                    }
                }
                Event::ControllerButtonDown { which, button, .. } => {
                    if let Some(gamepad) = self.gamepads.get_mut(&GamepadId(which)) {
                        gamepad.digital_inputs.activate(button);
                    }
                }
                Event::ControllerButtonUp { which, button, .. } => {
                    if let Some(gamepad) = self.gamepads.get_mut(&GamepadId(which)) {
                        gamepad.digital_inputs.deactivate(button);
                    }
                }
                _ => (),
            }
        }

        Ok(())
    }
}
