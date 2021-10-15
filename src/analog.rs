//! Generic analog input support.

use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/// The minimum value of an analog input.
pub const ANALOG_MIN: f32 = -1.0;
/// The maximum value of an analog input.
pub const ANALOG_MAX: f32 = 1.0;

/// Wrapper around `f32`.
#[derive(Default, Debug, Clone, Copy, PartialEq, PartialOrd)]
pub(crate) struct AnalogInputValue(f32);

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

impl From<AnalogInputValue> for f32 {
    fn from(value: AnalogInputValue) -> Self {
        value.0
    }
}

/// Container for analog inputs.
#[derive(Debug)]
pub struct AnalogInput<T> {
    inputs: HashMap<T, AnalogInputValue>,
    just_activated: HashSet<T>,
    just_deactivated: HashSet<T>,
    activation_threshold: AnalogInputValue,
}

impl<T> AnalogInput<T>
where
    T: Hash + Eq,
{
    /// Gets the value of an analog input.
    ///
    /// Defaults to returning `0.0` if the input has not been read yet.
    pub fn value(&self, input: T) -> f32 {
        if let Some(&value) = self.inputs.get(&input) {
            f32::from(value)
        } else {
            0.0
        }
    }

    /// Checks if an analog input just left the deadzone.
    pub fn just_activated(&self, input: T) -> Option<f32> {
        if self.just_activated.contains(&input) {
            Some(self.value(input))
        } else {
            None
        }
    }

    /// Checks if an analog input just entered the deadzone.
    pub fn just_deactivated(&self, input: T) -> bool {
        self.just_deactivated.contains(&input)
    }
}

impl<T> AnalogInput<T>
where
    T: Hash + Copy + Eq,
{
    pub(crate) fn set(&mut self, input: T, value: AnalogInputValue) {
        let old_value = self.inputs.insert(input, value);
        let value = f32::from(value);
        let activation_threshold = f32::from(self.activation_threshold);

        if let Some(old_value) = old_value {
            let old_value = f32::from(old_value);
            if value.abs() < activation_threshold {
                self.just_activated.remove(&input);
                if old_value.abs() >= activation_threshold {
                    self.just_deactivated.insert(input);
                }
            } else {
                self.just_deactivated.remove(&input);
                // It is possible for an analog input to completely pass through the deadzone
                // between updates. In that case, both the old and new values would exceed the
                // activation threshold, but they would have opposite signs.
                if old_value.abs() < activation_threshold || value.signum() != old_value.signum() {
                    self.just_activated.insert(input);
                }
            }
        } else if value.abs() >= activation_threshold {
            self.just_activated.insert(input);
            self.just_deactivated.remove(&input);
        }
    }

    pub(crate) fn update(&mut self) {
        self.just_activated.clear();
        self.just_deactivated.clear();
    }
}

impl<T> Default for AnalogInput<T> {
    fn default() -> Self {
        Self {
            inputs: Default::default(),
            just_activated: Default::default(),
            just_deactivated: Default::default(),
            activation_threshold: DEFAULT_ACTIVATION_THRESHOLD,
        }
    }
}

const DEFAULT_ACTIVATION_THRESHOLD: AnalogInputValue = AnalogInputValue(0.1);
