use crate::{
    base::{FromGil, RemoveGil, PyDefault},
    sim, Input, Mat3, Vec3, new_gil_default, new_gil,
};
use pyo3::prelude::*;

#[pyclass]
#[derive(Clone, Debug)]
pub struct Car {
    position: Py<Vec3>,
    velocity: Py<Vec3>,
    angular_velocity: Py<Vec3>,
    orientation: Py<Mat3>,
    supersonic: bool,
    jumped: bool,
    double_jumped: bool,
    on_ground: bool,
    demolished: bool,
    boost: i32,
    jump_timer: f32,
    dodge_timer: f32,
    boost_timer: f32,
    enable_jump_acceleration: bool,
    dodge_torque: Py<Vec3>,
    frame: i32,
    time: f32,
    body: sim::car::CarBody,
    state: sim::car::CarState,
    hitbox_widths: Py<Vec3>,
    hitbox_offset: Py<Vec3>,
    team: i32,
    id: i32,
    controls: Py<Input>,
    I: Py<Mat3>,
    invI: Py<Mat3>,
}

impl PyDefault for Car {
    #[inline]
    fn py_default(py: Python) -> PyResult<Self> {
        Ok(Self {
            position: new_gil_default!(Vec3, py),
            velocity: new_gil_default!(Vec3, py),
            angular_velocity: new_gil_default!(Vec3, py),
            orientation: new_gil_default!(Mat3, py),
            supersonic: false,
            jumped: false,
            double_jumped: false,
            on_ground: false,
            demolished: false,
            boost: 0,
            jump_timer: 0.,
            dodge_timer: 0.,
            boost_timer: 0.,
            enable_jump_acceleration: false,
            dodge_torque: new_gil_default!(Vec3, py),
            frame: 0,
            time: 0.,
            body: sim::car::CarBody::Octane,
            state: sim::car::CarState::OnGround,
            hitbox_widths: new_gil_default!(Vec3, py),
            hitbox_offset: new_gil_default!(Vec3, py),
            team: 0,
            id: 0,
            controls: new_gil_default!(Input, py),
            I: new_gil_default!(Mat3, py),
            invI: new_gil_default!(Mat3, py),
        })
    }
}

impl RemoveGil<sim::car::Car> for Car {
    #[inline]
    fn remove_gil(self, py: Python) -> sim::car::Car {
        sim::car::Car {
            position: self.position.remove_gil(py),
            velocity: self.velocity.remove_gil(py),
            angular_velocity: self.angular_velocity.remove_gil(py),
            orientation: self.orientation.remove_gil(py),
            supersonic: self.supersonic,
            jumped: self.jumped,
            double_jumped: self.double_jumped,
            on_ground: self.on_ground,
            demolished: self.demolished,
            boost: self.boost,
            jump_timer: self.jump_timer,
            dodge_timer: self.dodge_timer,
            boost_timer: self.boost_timer,
            enable_jump_acceleration: self.enable_jump_acceleration,
            dodge_torque: self.dodge_torque.remove_gil(py),
            frame: self.frame,
            time: self.time,
            body: self.body,
            state: self.state,
            hitbox_widths: self.hitbox_widths.remove_gil(py),
            hitbox_offset: self.hitbox_offset.remove_gil(py),
            team: self.team,
            id: self.id,
            controls: self.controls.remove_gil(py),
            I: self.I.remove_gil(py),
            invI: self.invI.remove_gil(py),
        }
    }
}

impl FromGil<sim::car::Car> for Car {
    #[inline]
    fn from_gil(py: Python, car: sim::car::Car) -> PyResult<Self> {
        Ok(Self {
            position: new_gil!(Vec3, py, car.position),
            velocity: new_gil!(Vec3, py, car.velocity),
            angular_velocity: new_gil!(Vec3, py, car.angular_velocity),
        })
    }
}

#[pymethods]
impl Car {
    #[new]
    #[inline]
    fn __new__(py: Python) -> PyResult<Self> {
        Self::py_default(py)
    }

    #[inline]
    fn step(&mut self, py: Python, in_: Input, dt: f32) {
        self.remove_gil(py).step(in_.into(), dt);
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}
