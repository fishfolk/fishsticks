cfg_if::cfg_if! {
    if #[cfg(feature = "sdl2")] {
        #[path = "backend/sdl2.rs"]
        mod implementation;
    } else if #[cfg(feature = "gilrs")] {
        #[path = "backend/gilrs.rs"]
        mod implementation;
    } else {
        #[path = "backend/dummy.rs"]
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
