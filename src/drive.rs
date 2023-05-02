use crate::{mech, Car, Input, Vec3};
use pyo3::prelude::*;
use std::fmt;

#[pyclass]
pub struct Drive {
    #[pyo3(get, set)]
    car: Py<Car>,
    inner: mech::drive::Drive,
}

impl fmt::Debug for Drive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Drive")
            .field("speed", &self.inner.speed)
            .field("target", &self.inner.target)
            .field("controls", &self.inner.controls)
            .field("finished", &self.inner.finished)
            .finish()
    }
}

#[pymethods]
impl Drive {
    #[new]
    #[inline]
    fn __new__(car: Py<Car>) -> Self {
        Self {
            car,
            inner: mech::drive::Drive::default(),
        }
    }

    #[inline]
    fn step(&mut self, py: Python, dt: f32) {
        self.inner.step(&self.car.borrow(py).into(), dt);
    }

    #[inline]
    #[getter(speed)]
    fn get_speed(&self) -> f32 {
        self.inner.speed
    }

    #[inline]
    #[setter(speed)]
    fn set_speed(&mut self, speed: f32) {
        self.inner.speed = speed;
    }

    #[inline]
    #[getter(target)]
    fn get_target(&self) -> Vec3 {
        self.inner.target.into()
    }

    #[inline]
    #[setter(target)]
    fn set_target(&mut self, target: Vec3) {
        self.inner.target = target.into();
    }

    #[inline]
    #[getter(controls)]
    fn get_controls(&self) -> Input {
        self.inner.controls.into()
    }

    #[inline]
    #[getter(finished)]
    fn get_finished(&self) -> bool {
        self.inner.finished
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}
