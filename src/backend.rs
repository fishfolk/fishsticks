cfg_if::cfg_if! {
    if #[cfg(feature = "gilrs")] {
        compile_error!("gilrs not yet implemented");
    } else {
        // SDL2 has the highest compatibility of all game input libraries,
        // so it should be the default implementation.
        #[path = "backend/sdl2.rs"]
        mod implementation;
    }
}
pub use implementation::*;

use std::collections::HashMap;

pub trait GamepadSystem {
    fn update(&mut self, gamepads: &mut HashMap<GamepadId, super::Gamepad>) -> Result<(), String>;
}

pub trait Detachable {
    fn connected(&self) -> bool;
}
