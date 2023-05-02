use crate::{sim, Vec3};
use pyo3::{prelude::*, types::PyTuple};

#[pyclass(get_all, set_all)]
#[derive(Clone, Copy, Debug)]
pub struct Ball {
    time: f32,
    position: Vec3,
    velocity: Vec3,
    angular_velocity: Vec3,
}

impl Default for Ball {
    #[inline]
    fn default() -> Self {
        Self {
            time: 0.,
            position: Vec3::from([0., 0., 110.]),
            velocity: Vec3::default(),
            angular_velocity: Vec3::default(),
        }
    }
}

impl From<sim::ball::Ball> for Ball {
    #[inline]
    fn from(ball: sim::ball::Ball) -> Self {
        Self {
            time: ball.time,
            position: ball.position.into(),
            velocity: ball.velocity.into(),
            angular_velocity: ball.angular_velocity.into(),
        }
    }
}

impl From<Ball> for sim::ball::Ball {
    #[inline]
    fn from(ball: Ball) -> Self {
        Self {
            time: ball.time,
            position: ball.position.into(),
            velocity: ball.velocity.into(),
            angular_velocity: ball.angular_velocity.into(),
        }
    }
}

#[pymethods]
impl Ball {
    const NAMES: [&str; 4] = ["time", "position", "velocity", "angular_velocity"];

    #[new]
    #[pyo3(signature = (*args, **kwargs))]
    fn __new__(args: &PyTuple, kwargs: Option<&PyAny>) -> Self {
        if let Ok(args) = args.get_item(0).and_then(PyAny::extract) {
            return args;
        }

        let mut vec = [None; Self::NAMES.len() - 1];

        let mut time = args.get_item(0).and_then(PyAny::extract).ok();

        if let Ok(args) = args.extract::<Vec<Vec3>>() {
            vec.iter_mut().zip(args.into_iter().skip(1)).for_each(|(a, b)| *a = Some(b));
        } else {
            for (a, b) in vec.iter_mut().zip(args.into_iter().skip(1)) {
                if let Ok(x) = b.extract() {
                    *a = Some(x);
                }
            }
        }

        if let Some(kwargs) = kwargs {
            if let Ok(arg) = kwargs.get_item(Self::NAMES[0]).and_then(PyAny::extract) {
                time = Some(arg);
            }

            for (a, b) in vec.iter_mut().zip(Self::NAMES.into_iter().skip(1)) {
                if let Ok(x) = kwargs.get_item(b).and_then(PyAny::extract) {
                    *a = Some(x);
                }
            }
        }

        // if there are no items in vec that are Some, then we can just return the default
        if vec.iter().all(std::option::Option::is_none) {
            Self::default()
        } else {
            Ball {
                time: time.unwrap_or_default(),
                position: vec[0].unwrap_or_default(),
                velocity: vec[1].unwrap_or_default(),
                angular_velocity: vec[2].unwrap_or_default(),
            }
        }
    }

    fn step(&mut self, dt: f32) {
        // this code might look like a crime against humanity
        // and I won't deny that but the performance impact is negligible
        // it's well optimized by the compiler and makes syntax cleaner elsewhere
        let mut ball: sim::ball::Ball = (*self).into();
        ball.step(dt);
        *self = ball.into();
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn __repr__(&self) -> String {
        format!(
            "Ball(time={}, position={}, velocity={}, angular_velocity={})",
            self.time,
            self.position.__repr__(),
            self.velocity.__repr__(),
            self.angular_velocity.__repr__()
        )
    }
}
