use crate::sim;
use pyo3::prelude::*;

#[pyclass(get_all, set_all)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Input {
    steer: f32,
    roll: f32,
    pitch: f32,
    yaw: f32,
    throttle: f32,
    jump: bool,
    boost: bool,
    handbrake: bool,
    use_item: bool,
}

impl From<sim::input::Input> for Input {
    #[inline]
    fn from(input: sim::input::Input) -> Self {
        Self {
            steer: input.steer,
            roll: input.roll,
            pitch: input.pitch,
            yaw: input.yaw,
            throttle: input.throttle,
            jump: input.jump,
            boost: input.boost,
            handbrake: input.handbrake,
            use_item: input.use_item,
        }
    }
}

impl From<Input> for sim::input::Input {
    #[inline]
    fn from(input: Input) -> Self {
        Self {
            steer: input.steer,
            roll: input.roll,
            pitch: input.pitch,
            yaw: input.yaw,
            throttle: input.throttle,
            jump: input.jump,
            boost: input.boost,
            handbrake: input.handbrake,
            use_item: input.use_item,
        }
    }
}

#[pymethods]
impl Input {
    #[new]
    #[inline]
    fn __new__() -> Self {
        Self::default()
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}
