//! System for handling gamepad input.
//!
//! API is unstable and may change at any time.
//! Uses SDL2 as the backend.

#![warn(missing_docs)]

pub mod analog;
pub mod digital;
pub mod error;

mod backend;

pub use backend::{Axis, Button};

use analog::AnalogInput;
use analog::{AnalogInputValue, Deadzone};
use backend::Backend;
use backend::{ImplementationContext, OwnedImplementationGamepad};
use digital::DigitalInput;
use std::collections::HashMap;

use error::Result;

/// The instance Id of a gamepad.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GamepadId(backend::ImplementationId);

/// Holds the state of a gamepad.
pub struct Gamepad {
    // Backends that produce owned gamepads, such as SDL2, need this to keep the gamepads open.
    // Backends that produce references to gamepads, such as gilrs, do not need this.
    #[allow(dead_code)]
    owned_internal_gamepad: Option<OwnedImplementationGamepad>,
    /// Analog inputs, such as thumbsticks.
    pub analog_inputs: AnalogInput<Axis>,
    /// Digital inputs, such as buttons.
    pub digital_inputs: DigitalInput<Button>,
}

impl Gamepad {
    fn new(owned_internal_gamepad: Option<OwnedImplementationGamepad>) -> Self {
        Self {
            owned_internal_gamepad,
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
    gamepad_system: ImplementationContext,
    gamepads: HashMap<GamepadId, Gamepad>,
}

impl GamepadContext {
    /// Initializes the gamepad context.
    pub fn init() -> Result<Self> {
        let gamepad_system = ImplementationContext::new()?;
        let gamepads = HashMap::new();

        Ok(Self {
            gamepad_system,
            gamepads,
        })
    }

    /// Gets a reference to a specific gamepad.
    ///
    /// Returns `None` if the gamepad is not found.
    pub fn gamepad(&self, id: GamepadId) -> Option<&Gamepad> {
        self.gamepads.get(&id)
    }

    /// Gets an iterator over all gamepads.
    pub fn gamepads(&self) -> impl Iterator<Item = (GamepadId, &Gamepad)> {
        self.gamepads.iter().map(|(&id, gamepad)| (id, gamepad))
    }

    /// Updates the state of all gamepads.
    pub fn update(&mut self) -> Result<()> {
        self.gamepad_system.update(&mut self.gamepads)
    }

    /// Sets the analog deadzone for all analog inputs.
    pub fn set_deadzone(&mut self, deadzone: f32) {
        for (_, gamepad) in self.gamepads.iter_mut() {
            let deadzone = Deadzone::from(AnalogInputValue::from(deadzone));
            gamepad.analog_inputs.set_deadzone(deadzone);
        }
    }

    /// Sets the digital deadzone for all analog inputs.
    pub fn set_deadzone_digital(&mut self, deadzone: f32) {
        for (_, gamepad) in self.gamepads.iter_mut() {
            let deadzone = Deadzone::from(AnalogInputValue::from(deadzone));
            gamepad.analog_inputs.set_deadzone_digital(deadzone);
        }
    }
}
