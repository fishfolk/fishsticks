//! Generic digital input support.

use std::collections::HashSet;
use std::hash::Hash;

/// Container for digital inputs.
#[derive(Debug)]
pub struct DigitalInput<T> {
    activated: HashSet<T>,
    just_activated: HashSet<T>,
    just_deactivated: HashSet<T>,
}

impl<T> DigitalInput<T>
where
    T: Hash + Eq,
{
    /// Checks if a digital input is activated.
    pub fn activated(&self, input: T) -> bool {
        self.activated.contains(&input)
    }

    /// Checks if a digital input has just been activated.
    pub fn just_activated(&self, input: T) -> bool {
        self.just_activated.contains(&input)
    }

    /// Checks if a digital input has just been deactivated.
    pub fn just_deactivated(&self, input: T) -> bool {
        self.just_deactivated.contains(&input)
    }
}

impl<T> DigitalInput<T>
where
    T: Hash + Copy + Eq,
{
    pub(crate) fn activate(&mut self, input: T) {
        if !self.activated(input) {
            self.activated.insert(input);
            self.just_activated.insert(input);
            self.just_deactivated.remove(&input);
        }
    }

    pub(crate) fn deactivate(&mut self, input: T) {
        if self.activated(input) {
            self.activated.remove(&input);
            self.just_activated.remove(&input);
            self.just_deactivated.insert(input);
        }
    }

    pub(crate) fn update(&mut self) {
        self.just_activated.clear();
        self.just_deactivated.clear();
    }
}

impl<T> Default for DigitalInput<T> {
    fn default() -> Self {
        Self {
            activated: Default::default(),
            just_activated: Default::default(),
            just_deactivated: Default::default(),
        }
    }
}
