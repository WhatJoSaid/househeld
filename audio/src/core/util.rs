use std::ops::{Range, RangeInclusive};

/// Enum used for determening the [Unit] of several widgets.
pub enum Unit {
    /// Units in Hz, using the range set.
    Hertz(Range<f32>),
    /// Units in dBs, using the range set.
    Decibel(Range<f32>),
    /// Units in Percent, from 0% to 100%.
    Percent,
    /// Units in Pan, from -50% to 50%
    Pan
}

/// A number between 0.0 and 1.0, useful for modulation/scaling.
pub struct AlphaRange {
    value: f64
}

impl Default for AlphaRange {
    fn default() -> Self {
        Self { value: 0.0 }
    }
}

impl AlphaRange {
    const RANGE: RangeInclusive<f64> = 0.0..=1.0;

    /// Create a new AlphaRange, bounded to values between 0.0 and 1.0
    pub fn new_bounded(value: f64) -> Self {
        if value > *Self::RANGE.end() {
            Self { value: 1.0 }
        } else if value < *Self::RANGE.start() {
            Self { value: 0.0 }
        } else {
            Self { value }
        }
    }

    /// Create a new AlphaRange, bounded to values that are scaled.
    pub fn new_scaled(value: f64, scale: f64) -> Self {
        Self::new_bounded(value / scale)
    }

    pub fn change_bounded(&mut self, value: f64) {
        if value > *Self::RANGE.end() {
            self.value = 1.0;
        } else if value < *Self::RANGE.start() {
            self.value = 0.0;
        } else {
            self.value = value;
        }
    }

    pub fn change_scaled(&mut self, value: f64, scale: f64)
    {
        self.change_bounded(value / scale);
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn get_value_scaled(&self, scale: f64) -> f64 {
        self.value * scale
    }
}