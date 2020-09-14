use splines::{
    interpolate::{cubic_bezier_def, quadratic_bezier_def, Interpolate, Linear},
    Interpolation, Key, Spline,
};
use std::time::Duration;

use bevy::prelude::*;

#[derive(Copy, Clone)]
pub struct TweenValue<T: Copy>(pub T);
pub struct Tween<T: Copy> {
    inner: Option<(Spline<f32, TweenValue<T>>, Timer)>
}

impl<T: Copy> Tween<T>
where
    TweenValue<T>: Linear<f32>,
{
    pub fn run(&mut self, start: T, end: T, duration: Duration) {
        let start = Key::new(0f32, TweenValue(start), Interpolation::Linear);
        let end = Key::new(1f32, TweenValue(end), Interpolation::default());
        self.inner = Some((Spline::from_vec(vec![start, end]), Timer::new(duration, false)));
    }

    pub fn target(&self) -> Option<T> {
        self.inner.as_ref().map(|(spline, _)| spline.clamped_sample(1.0).unwrap().0)
    }

    pub fn retarget(&mut self, target: T)
    where
        T: PartialEq,
    {
        if let Some((spline, timer)) = &mut self.inner {
            let end = spline.clamped_sample(1.0).unwrap().0;
            if target != end {
                let progress = timer.elapsed / timer.duration;
                let new_start = Key::new(
                    progress,
                    TweenValue(spline.clamped_sample(progress).unwrap().0),
                    Interpolation::Linear,
                );
                let new_end = Key::new(1f32, TweenValue(target), Interpolation::Linear);
                *spline = Spline::from_vec(vec![new_start, new_end]);
            }
        }
    }
}

impl<T: Copy> Default for Tween<T> {
    fn default() -> Self {
        Self { inner: None, }
    }
}

pub fn tween_system<T: Copy + Component>(
    time: Res<Time>,
    mut tween: Mut<Tween<T>>,
    mut object: Mut<T>,
) where
    TweenValue<T>: Interpolate<f32>,
{
    let remove = if let Some((spline, timer)) = &mut tween.inner {
        timer.tick(time.delta_seconds);
        let progress = timer.elapsed / timer.duration;
        *object = spline.clamped_sample(progress).unwrap().0;
        timer.finished
    } else {
        false
    };

    if remove {
        tween.inner = None;
    }

}

impl<T: Copy> Interpolate<f32> for TweenValue<T>
where
    TweenValue<T>: Linear<f32>,
{
    fn lerp(a: Self, b: Self, t: f32) -> Self { a.outer_mul(1. - t) + b.outer_mul(t) }
    fn quadratic_bezier(a: Self, u: Self, b: Self, t: f32) -> Self {
        quadratic_bezier_def(a, u, b, t)
    }
    fn cubic_bezier(a: Self, u: Self, v: Self, b: Self, t: f32) -> Self {
        cubic_bezier_def(a, u, v, b, t)
    }
}
