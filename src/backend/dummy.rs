compile_error!("no gamepad backend chosen");

// The sole purpose of everything below this comment is to supress
// irrelevant warnings and errors. All of it is dead code.

use crate::{Gamepad, GamepadId};
use std::collections::HashMap;

use crate::Result;

/// Dummy axis.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {}

/// Dummy button.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Button {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImplementationId {}

pub enum OwnedImplementationGamepad {}

pub struct ImplementationContext;

impl ImplementationContext {
    pub fn new() -> Result<Self> {
        Err("Dummy context".into())
    }
}

impl super::Backend for ImplementationContext {
    fn update(&mut self, _: &mut HashMap<GamepadId, Gamepad>) -> Result<()> {
        Err("Dummy context".into())
    }
}
