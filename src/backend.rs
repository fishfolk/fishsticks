cfg_if::cfg_if! {
    if #[cfg(feature = "gilrs")] {
        #[path = "backend/gilrs.rs"]
        mod implementation;
    } else {
        // SDL2 has the highest compatibility of all game input libraries,
        // so it should be the default implementation.
        #[path = "backend/sdl2.rs"]
        mod implementation;
    }
}

pub use implementation::*;

use crate::{Gamepad, GamepadId};
use std::collections::HashMap;

use crate::Result;

pub trait Backend {
    fn update(&mut self, gamepads: &mut HashMap<GamepadId, Gamepad>) -> Result<()>;
}
