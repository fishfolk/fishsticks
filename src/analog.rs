//! Generic analog input support.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// The minimum value of an analog input.
pub const ANALOG_MIN: f32 = -1.0;
/// The maximum value of an analog input.
pub const ANALOG_MAX: f32 = 1.0;

/// Wrapper around `f32` for analog inputs.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub(crate) struct AnalogInputValue(f32);

impl AnalogInputValue {
    fn get(&self) -> f32 {
        self.0
    }
}

impl From<i16> for AnalogInputValue {
    fn from(value: i16) -> Self {
        let analog_value = value as f32 / i16::MAX as f32;
        Self(analog_value.clamp(ANALOG_MIN, ANALOG_MAX))
    }
}

impl From<f32> for AnalogInputValue {
    fn from(value: f32) -> Self {
        if value.is_finite() {
            Self(value.clamp(ANALOG_MIN, ANALOG_MAX))
        } else {
            Self(0.0)
        }
    }
}

/// Wrapper around `f32` for deadzones.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub(crate) struct Deadzone(f32);

impl Deadzone {
    fn get(&self) -> f32 {
        self.0
    }
}

impl From<AnalogInputValue> for Deadzone {
    fn from(value: AnalogInputValue) -> Self {
        Self(value.0.abs())
    }
}

/// Container for analog inputs.
#[derive(Debug)]
pub struct AnalogInput<T> {
    inputs: HashMap<T, AnalogInputValue>,

    just_activated: HashSet<T>,
    just_deactivated: HashSet<T>,
    deadzone: Deadzone,

    just_activated_digital: HashSet<T>,
    just_deactivated_digital: HashSet<T>,
    digital_deadzone: Deadzone,
}

impl<T> AnalogInput<T>
where
    T: Hash + Eq,
{
    /// Gets the value of an analog input.
    ///
    /// Returns `0.0` if the input is within the analog deadzone, or if it has not been read yet.
    pub fn value(&self, input: T) -> f32 {
        match self.inputs.get(&input) {
            Some(&value) if Deadzone::from(value) >= self.deadzone => {
                let deadzone = self.deadzone.get();
                let remapped_value = (value.get().abs() - deadzone) / (ANALOG_MAX - deadzone);
                value.get().signum() * remapped_value
            }
            _ => 0.0,
        }
    }

    /// Checks if an analog input just left the analog deadzone.
    pub fn just_activated(&self, input: T) -> Option<f32> {
        if self.just_activated.contains(&input) {
            Some(self.value(input))
        } else {
            None
        }
    }

    /// Checks if an analog input just entered the analog deadzone.
    pub fn just_deactivated(&self, input: T) -> bool {
        self.just_deactivated.contains(&input)
    }

    /// Converts an analog input to a digital value.
    ///
    /// Returns either `ANALOG_MIN` or `ANALOG_MAX` when a nonzero input is outside
    /// the digital deadzone, and `0.0` otherwise.
    pub fn digital_value(&self, input: T) -> f32 {
        match self.inputs.get(&input) {
            Some(&value) if Deadzone::from(value) >= self.digital_deadzone => {
                if value.get() < 0.0 {
                    ANALOG_MIN
                } else if value.get() > 0.0 {
                    ANALOG_MAX
                } else {
                    0.0
                }
            }
            _ => 0.0,
        }
    }

    /// Checks if an analog input just left the digital deadzone.
    pub fn just_activated_digital(&self, input: T) -> Option<f32> {
        if self.just_activated_digital.contains(&input) {
            Some(self.digital_value(input))
        } else {
            None
        }
    }

    /// Checks if an analog input just entered the digital deadzone.
    pub fn just_deactivated_digital(&self, input: T) -> bool {
        self.just_deactivated_digital.contains(&input)
    }
}

impl<T> AnalogInput<T>
where
    T: Hash + Copy + Eq,
{
    pub(crate) fn set(&mut self, input: T, value: AnalogInputValue) {
        let old_value = self.inputs.insert(input, value);
        let value = value.get();
        let deadzone = self.deadzone.get();
        let digital_deadzone = self.digital_deadzone.get();

        if let Some(old_value) = old_value {
            let old_value = old_value.get();

            if value.abs() < deadzone {
                self.just_activated.remove(&input);
                if old_value.abs() >= deadzone {
                    self.just_deactivated.insert(input);
                }
            } else {
                self.just_deactivated.remove(&input);
                // It is possible for an analog input to completely pass through the deadzone
                // between updates. In that case, both the old and new values would exceed the
                // deadzone, but they would have opposite signs.
                if old_value.abs() < deadzone || value.signum() != old_value.signum() {
                    self.just_activated.insert(input);
                }
            }

            if value.abs() < digital_deadzone {
                self.just_activated_digital.remove(&input);
                if old_value.abs() >= digital_deadzone {
                    self.just_deactivated_digital.insert(input);
                }
            } else {
                self.just_deactivated_digital.remove(&input);
                if old_value.abs() < digital_deadzone || value.signum() != old_value.signum() {
                    self.just_activated_digital.insert(input);
                }
            }
        } else {
            if value.abs() >= deadzone {
                self.just_activated.insert(input);
                self.just_deactivated.remove(&input);
            }
            if value.abs() >= digital_deadzone {
                self.just_activated_digital.insert(input);
                self.just_deactivated_digital.remove(&input);
            }
        }
    }

    pub(crate) fn update(&mut self) {
        self.just_activated.clear();
        self.just_deactivated.clear();
        self.just_activated_digital.clear();
        self.just_deactivated_digital.clear();
    }

    pub(crate) fn set_deadzone(&mut self, deadzone: Deadzone) {
        self.deadzone = deadzone;
    }

    pub(crate) fn set_digital_deadzone(&mut self, deadzone: Deadzone) {
        self.digital_deadzone = deadzone;
    }
}

impl<T> Default for AnalogInput<T> {
    fn default() -> Self {
        Self {
            inputs: Default::default(),

            just_activated: Default::default(),
            just_deactivated: Default::default(),
            deadzone: DEFAULT_DEADZONE,

            just_activated_digital: Default::default(),
            just_deactivated_digital: Default::default(),
            digital_deadzone: DEFAULT_DEADZONE_DIGITAL,
        }
    }
}

const DEFAULT_DEADZONE: Deadzone = Deadzone(0.1);
const DEFAULT_DEADZONE_DIGITAL: Deadzone = Deadzone(0.5);
