use splines::{
    interpolate::{cubic_bezier_def, quadratic_bezier_def, Interpolate, Linear},
    Interpolation, Key, Spline,
};
use std::{
    ops::{Add, Sub},
    time::Duration,
};

use bevy::prelude::*;

#[derive(Copy, Clone)]
pub struct TweenValue<T: Copy>(pub T);
pub struct TweenComponent<T: Copy> {
    spline: Spline<f32, TweenValue<T>>,
    timer: Timer,
}

impl<T: Copy> TweenComponent<T>
where TweenValue<T>: Linear<f32>
{
    pub fn end(&self) -> T { self.spline.clamped_sample(1.1).unwrap().0 }
}

pub trait Tween: Sized + Copy {
    fn tween(start: Self, end: Self, duration: Duration) -> TweenComponent<Self> {
        let start = Key::new(0., TweenValue(start), Interpolation::Linear);
        let end = Key::new(1., TweenValue(end), Interpolation::default());
        TweenComponent {
            spline: Spline::from_vec(vec![start, end]),
            timer: Timer::new(duration, false),
        }
    }
    fn tween_to(self, target: Self, duration: Duration) -> TweenComponent<Self> {
        Self::tween(self, target, duration)
    }
}

impl<T: Copy> Tween for T where TweenValue<T>: Interpolate<f32> {}

pub fn tween_system<T: Tween + Component>(
    mut commands: Commands,
    time: Res<Time>,
    entity: Entity,
    mut tween: Mut<TweenComponent<T>>,
    mut object: Mut<T>,
) where
    TweenValue<T>: Interpolate<f32>,
{
    tween.timer.tick(time.delta_seconds);
    let progress = tween.timer.elapsed / tween.timer.duration;
    *object = tween.spline.clamped_sample(progress).unwrap().0;
    if tween.timer.finished {
        commands.remove_one::<TweenComponent<T>>(entity);
    }
}

impl<T: Copy> Interpolate<f32> for TweenValue<T>
    where TweenValue<T>: Linear<f32>,
{
    fn lerp(a: Self, b: Self, t: f32) -> Self { a.outer_mul(1. - t) + b.outer_mul(t) }
    fn quadratic_bezier(a: Self, u: Self, b: Self, t: f32) -> Self { quadratic_bezier_def(a, u, b, t) }
    fn cubic_bezier(a: Self, u: Self, v: Self, b: Self, t: f32) -> Self { cubic_bezier_def(a, u, v, b, t) }
}

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
