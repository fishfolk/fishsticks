pub use gilrs::{Axis, Button};

use crate::analog::AnalogInputValue;
use crate::{Gamepad, GamepadId};
use std::collections::HashMap;

use crate::Result;

pub type ImplementationId = gilrs::GamepadId;

impl From<GamepadId> for usize {
    fn from(id: GamepadId) -> Self {
        id.0.into()
    }
}

pub enum OwnedImplementationGamepad {}

pub struct ImplementationContext {
    context: gilrs::Gilrs,
    init_gamepads: Vec<GamepadId>,
}

impl ImplementationContext {
    pub fn new() -> Result<Self> {
        match gilrs::Gilrs::new() {
            Ok(context) => {
                let mut init_gamepads = Vec::new();

                for (gamepad_id, _) in context.gamepads() {
                    init_gamepads.push(GamepadId(gamepad_id));
                }

                Ok(Self {
                    context,
                    init_gamepads,
                })
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

impl super::Backend for ImplementationContext {
    fn update(&mut self, gamepads: &mut HashMap<GamepadId, Gamepad>) -> Result<()> {
        for gamepad in self.init_gamepads.drain(..) {
            gamepads.insert(gamepad, Gamepad::new(None));
        }

        for (_, gamepad) in gamepads.iter_mut() {
            gamepad.update_inputs();
        }

        while let Some(gilrs::Event { id, event, .. }) = self.context.next_event() {
            use gilrs::EventType;
            match event {
                EventType::Connected => {
                    gamepads.insert(GamepadId(id), Gamepad::new(None));

                    #[cfg(debug_assertions)]
                    println!("Added gamepad \"{}\"", self.context.gamepad(id).name());
                }
                EventType::Disconnected => {
                    gamepads.remove(&GamepadId(id));

                    #[cfg(debug_assertions)]
                    println!("Removed gamepad \"{}\"", self.context.gamepad(id).name());
                }
                EventType::AxisChanged(axis, value, _) => {
                    if let Some(gamepad) = gamepads.get_mut(&GamepadId(id)) {
                        gamepad
                            .analog_inputs
                            .set(axis, AnalogInputValue::from(value));
                    }
                }
                EventType::ButtonPressed(button, _) => {
                    if let Some(gamepad) = gamepads.get_mut(&GamepadId(id)) {
                        gamepad.digital_inputs.activate(button);
                    }
                }
                EventType::ButtonReleased(button, _) => {
                    if let Some(gamepad) = gamepads.get_mut(&GamepadId(id)) {
                        gamepad.digital_inputs.deactivate(button);
                    }
                }
                _ => (),
            }
        }

        Ok(())
    }
}
