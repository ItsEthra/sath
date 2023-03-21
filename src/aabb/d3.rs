use crate::{Float, Vector3};
use std::mem::swap;

type V3<F> = Vector3<F>;

/// 3D Axis aligned bounded box.
#[derive(Debug, Clone, Copy)]
pub struct Aabb3<F: Float> {
    /// Min point.
    pub min: V3<F>,
    /// Max point.
    pub max: V3<F>,
}

impl<F: Float> Aabb3<F> {
    /// Creates `Aabb` from min, max vectors.
    pub fn from_min_max(min: V3<F>, max: V3<F>) -> Self {
        Self { min, max }
    }

    /// Translates bounding box by some delta.
    pub fn translate(&mut self, delta: V3<F>) {
        self.min += delta;
        self.max += delta;
    }

    /// Returns translated copy of the `Aabb`.
    pub fn translated(self, delta: V3<F>) -> Self {
        Self::from_min_max(self.min + delta, self.max + delta)
    }

    /// Checks if `Aabb` is right, i.e. `max` > `min`.
    pub fn is_right(&self) -> bool {
        self.max > self.min
    }

    /// Swaps `min`, `max`
    pub fn inverse(&mut self) {
        swap(&mut self.min, &mut self.max)
    }

    /// Returns inversed copy of `Aabb`, i.e. with `min`, `max` swapped.
    pub fn inversed(self) -> Self {
        Self::from_min_max(self.max, self.min)
    }

    /// Returns the volume of the bounding box.
    pub fn volume(&self) -> F {
        let dv = self.max - self.min;
        dv.product()
    }

    /// Checks if `Aabb` contains a point.
    pub fn contains(&self, point: V3<F>) -> bool {
        point >= self.min && point <= self.max
    }
}
