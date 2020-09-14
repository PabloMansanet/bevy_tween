use bevy::prelude::*;
use splines::interpolate::Linear;
use std::ops::{Add, Sub};

use crate::TweenValue;

impl Linear<f32> for TweenValue<Translation> {
    fn outer_mul(self, t: f32) -> Self { Self(Translation::from((self.0).0 * t)) }
    fn outer_div(self, t: f32) -> Self { Self(Translation::from((self.0).0 / t)) }
}

impl Add for TweenValue<Translation> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output { Self(Translation::from((self.0).0 + (rhs.0).0)) }
}

impl Sub for TweenValue<Translation> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output { Self(Translation::from((self.0).0 - (rhs.0).0)) }
}

impl Linear<f32> for TweenValue<Color> {
    fn outer_mul(self, t: f32) -> Self { Self(self.0 * t) }
    fn outer_div(self, t: f32) -> Self { Self(self.0 * t) }
}

impl Add for TweenValue<Color> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output { Self(self.0 + rhs.0) }
}

impl Sub for TweenValue<Color> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self(Color::rgba(
            (self.0.r - rhs.0.r).clamp(0.0, 1.0),
            (self.0.g - rhs.0.g).clamp(0.0, 1.0),
            (self.0.b - rhs.0.b).clamp(0.0, 1.0),
            (self.0.a - rhs.0.a).clamp(0.0, 1.0),
        ))
    }
}
