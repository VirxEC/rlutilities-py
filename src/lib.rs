mod pytypes;

use std::fmt;

use pyo3::{exceptions::PyIndexError, prelude::*, pyclass::CompareOp, types::PyTuple, wrap_pyfunction, wrap_pymodule};
use pytypes::{FieldInfoPacket, GameTickPacket};
pub use rlutilities_rs::{cmat3, cvec3, linalg, mech, rlu, sim};

macro_rules! pynamedmodule {
    (doc: $doc:literal, name: $name:tt, funcs: [$($func_name:path),*], classes: [$($class_name:ident),*], submodules: [$($submodule_name:ident),*]) => {
        #[doc = $doc]
        #[pymodule]
        #[allow(redundant_semicolons)]
        fn $name(_py: Python, m: &PyModule) -> PyResult<()> {
            $(m.add_function(wrap_pyfunction!($func_name, m)?)?);*;
            $(m.add_class::<$class_name>()?);*;
            $(m.add_wrapped(wrap_pymodule!($submodule_name))?);*;
            Ok(())
        }
    };
}

#[pyclass(get_all, set_all)]
#[derive(Clone, Copy, Debug, Default)]
struct Input {
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

#[pyclass]
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
struct Car(sim::car::Car);

impl fmt::Debug for Car {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Car")
            .field("position", &self.0.position)
            .field("velocity", &self.0.velocity)
            .field("angular_velocity", &self.0.angular_velocity)
            .field("orientation", &self.0.orientation)
            .field("boost", &self.0.boost)
            .field("jumped", &self.0.jumped)
            .field("double_jumped", &self.0.double_jumped)
            .field("on_ground", &self.0.on_ground)
            .field("supersonic", &self.0.supersonic)
            .field("demolished", &self.0.demolished)
            .finish()
    }
}

impl From<sim::car::Car> for Car {
    #[inline]
    fn from(car: sim::car::Car) -> Self {
        Self(car)
    }
}

impl From<Car> for sim::car::Car {
    #[inline]
    fn from(car: Car) -> Self {
        car.0
    }
}

#[pymethods]
impl Car {
    #[new]
    #[inline]
    fn __new__() -> Self {
        Self::default()
    }

    #[inline]
    fn step(&mut self, in_: Input, dt: f32) {
        self.0.step(in_.into(), dt);
    }

    #[getter(position)]
    #[inline]
    fn get_position(&self) -> Vec3 {
        self.0.position.into()
    }

    #[setter(position)]
    #[inline]
    fn set_position(&mut self, position: Vec3) {
        self.0.position = position.into();
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

#[pyclass]
#[derive(Default)]
#[repr(transparent)]
struct Game(sim::game::Game);

impl Game {
    #[inline]
    fn get_mut_pads(&mut self) -> impl Iterator<Item = &mut sim::boost_pad::BoostPad> {
        self.0.pads.pin_mut().iter_mut().map(std::pin::Pin::get_mut)
    }

    #[inline]
    fn get_mut_goals(&mut self) -> impl Iterator<Item = &mut sim::goal::Goal> {
        self.0.goals.pin_mut().iter_mut().map(std::pin::Pin::get_mut)
    }

    #[inline]
    fn get_mut_cars(&mut self) -> impl Iterator<Item = &mut sim::car::Car> {
        self.0.cars.pin_mut().iter_mut().map(std::pin::Pin::get_mut)
    }
}

#[pymethods]
impl Game {
    #[new]
    #[inline]
    fn __new__() -> Self {
        Self::default()
    }

    #[inline]
    #[staticmethod]
    fn set_mode(mode: String) {
        sim::game::Game::set_mode(mode);
    }

    fn read_field_info(&mut self, field_info: FieldInfoPacket) {
        for (cpad, new_pad) in self.get_mut_pads().zip(field_info.cpads()) {
            *cpad = new_pad;
        }

        for (cgoal, new_goal) in self.get_mut_goals().zip(field_info.cgoals()) {
            *cgoal = new_goal;
        }
    }

    fn read_packet(&mut self, packet: GameTickPacket) {
        // update game info
        self.0.time_delta = packet.game_info.seconds_elapsed - self.0.time;
        self.0.time = packet.game_info.seconds_elapsed;
        self.0.time_remaining = packet.game_info.game_time_remaining;
        self.0.gravity.data[2] = packet.game_info.world_gravity_z;

        self.0.state = if packet.game_info.is_match_ended {
            sim::game::GameState::Ended
        } else if packet.game_info.is_round_active {
            if packet.game_info.is_kickoff_pause {
                sim::game::GameState::Kickoff
            } else {
                sim::game::GameState::Active
            }
        } else {
            sim::game::GameState::Inactive
        };

        // update boost pads
        for (cpad, pad) in self.get_mut_pads().zip(packet.boostpads()) {
            cpad.state = pad.is_active.into();
            cpad.timer = pad.timer;
        }

        // add or remove cars to match the number in the packet
        match packet.cars().len().cmp(&self.0.cars.len()) {
            std::cmp::Ordering::Greater => {
                for _ in self.0.cars.len()..packet.cars().len() {
                    self.0.cars.pin_mut().push(sim::car::Car::default());
                }
            }
            std::cmp::Ordering::Less => {
                for _ in packet.cars().len()..self.0.cars.len() {
                    self.0.cars.pin_mut().pop();
                }
            }
            _ => {}
        }

        // update cars
        for (car, ccar) in packet.cars().iter().zip(self.get_mut_cars()) {
            ccar.time = packet.game_info.seconds_elapsed;
            ccar.position = car.physics.location.into();
            ccar.velocity = car.physics.velocity.into();
            ccar.angular_velocity = car.physics.angular_velocity.into();
            ccar.orientation = linalg::math::euler_to_rotation(&car.physics.rotation.into());
            ccar.demolished = car.is_demolished;
            ccar.on_ground = car.has_wheel_contact;
            ccar.supersonic = car.is_super_sonic;
            ccar.jumped = car.jumped;
            ccar.double_jumped = car.double_jumped;
            ccar.team = car.team;
            ccar.boost = car.boost;
            ccar.hitbox_widths = car.hitbox.into();
            ccar.hitbox_offset = car.hitbox_offset.into();
            ccar.id = car.spawn_id;
        }

        // update ball
        self.0.ball.time = packet.game_info.seconds_elapsed;
        self.0.ball.position = packet.game_ball.physics.location.into();
        self.0.ball.velocity = packet.game_ball.physics.velocity.into();
        self.0.ball.angular_velocity = packet.game_ball.physics.angular_velocity.into();
    }

    #[inline]
    #[getter(ball)]
    fn get_ball(&self) -> Ball {
        self.0.ball.into()
    }

    #[inline]
    #[setter(ball)]
    fn set_ball(&mut self, ball: Ball) {
        self.0.ball = ball.into();
    }

    #[inline]
    #[getter(cars)]
    fn get_cars(&self) -> Vec<Car> {
        self.0.cars.iter().cloned().map(Into::into).collect()
    }

    #[inline]
    #[getter(time_delta)]
    fn get_time_delta(&self) -> f32 {
        self.0.time_delta
    }
}

#[pyclass(get_all, set_all)]
#[derive(Clone, Copy, Debug)]
struct Ball {
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
            position: Vec3([0., 0., 110.]),
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

#[pyclass]
#[derive(Clone, Copy, PartialEq, PartialOrd, Default, Debug)]
#[pyo3(name = "mat3")]
struct Mat3([f32; 9]);

impl From<cmat3> for Mat3 {
    #[inline]
    fn from(value: cmat3) -> Self {
        Self(value.data)
    }
}

impl From<Mat3> for cmat3 {
    #[inline]
    fn from(value: Mat3) -> Self {
        Self { data: value.0 }
    }
}

#[pyclass]
#[derive(Clone, Copy, PartialEq, PartialOrd, Default, Debug)]
#[pyo3(name = "vec3")]
#[repr(transparent)]
struct Vec3([f32; 3]);

impl From<cvec3> for Vec3 {
    #[inline]
    fn from(value: cvec3) -> Self {
        Self(value.data)
    }
}

impl From<Vec3> for cvec3 {
    #[inline]
    fn from(value: Vec3) -> Self {
        Self { data: value.0 }
    }
}

#[pymethods]
impl Vec3 {
    const NAMES: [&str; 3] = ["x", "y", "z"];

    #[new]
    #[pyo3(signature = (*args, **kwargs))]
    fn new(args: &PyTuple, kwargs: Option<&PyAny>) -> Self {
        if let Ok(args) = args.get_item(0).and_then(PyAny::extract) {
            return args;
        }

        let mut vec = [None; Self::NAMES.len()];

        if let Ok(args) = args.get_item(0).and_then(PyAny::extract::<Vec<f32>>) {
            vec.iter_mut().zip(args.into_iter()).for_each(|(a, b)| *a = Some(b));
        } else if let Ok(args) = args.extract::<Vec<f32>>() {
            vec.iter_mut().zip(args.into_iter()).for_each(|(a, b)| *a = Some(b));
        } else {
            for (a, b) in vec.iter_mut().zip(args.into_iter()) {
                if let Ok(x) = b.extract() {
                    *a = Some(x);
                }
            }
        }

        if let Some(kwargs) = kwargs {
            for (a, b) in vec.iter_mut().zip(Self::NAMES.into_iter()) {
                if let Ok(x) = kwargs.get_item(b).and_then(PyAny::extract) {
                    *a = Some(x);
                }
            }
        }

        Self([vec[0].unwrap_or_default(), vec[1].unwrap_or_default(), vec[2].unwrap_or_default()])
    }

    #[inline]
    fn __getitem__(&self, index: usize) -> PyResult<f32> {
        if index >= Self::NAMES.len() {
            Err(PyIndexError::new_err("index out of range"))
        } else {
            Ok(self.0[index])
        }
    }

    #[inline]
    fn __setitem__(&mut self, index: usize, value: f32) -> PyResult<()> {
        if index >= Self::NAMES.len() {
            Err(PyIndexError::new_err("index out of range"))
        } else {
            self.0[index] = value;
            Ok(())
        }
    }

    #[inline]
    #[getter(x)]
    fn get_x(&self) -> f32 {
        self.0[0]
    }

    #[inline]
    #[setter(x)]
    fn set_x(&mut self, x: f32) {
        self.0[0] = x;
    }

    #[inline]
    #[getter(y)]
    fn get_y(&self) -> f32 {
        self.0[1]
    }

    #[inline]
    #[setter(y)]
    fn set_y(&mut self, y: f32) {
        self.0[1] = y;
    }

    #[inline]
    #[getter(z)]
    fn get_z(&self) -> f32 {
        self.0[2]
    }

    #[inline]
    #[setter(z)]
    fn set_z(&mut self, z: f32) {
        self.0[2] = z;
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }

    #[inline]
    fn __repr__(&self) -> String {
        format!("vec3(x={}, y={}, z={})", self.0[0], self.0[1], self.0[2])
    }

    /// Only == and != are actually supported right now
    fn __richcmp__(&self, other: Self, op: CompareOp) -> bool {
        if !matches!(op, CompareOp::Eq | CompareOp::Ne) {
            return false;
        };

        let Some(cmp) = self.partial_cmp(&other) else {
            return false;
        };

        op.matches(cmp)
    }
}

#[pyclass]
struct Field();

#[pyclass]
#[derive(Clone, Copy, Default)]
#[repr(transparent)]
struct Drive(mech::drive::Drive);

impl fmt::Debug for Drive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Drive")
            .field("speed", &self.0.speed)
            .field("target", &self.0.target)
            .field("controls", &self.0.controls)
            .field("finished", &self.0.finished)
            .finish()
    }
}

#[pymethods]
impl Drive {
    #[new]
    #[inline]
    fn __new__(car: Car) -> Self {
        Self(mech::drive::Drive::new(car.0))
    }

    #[inline]
    fn step(&mut self, dt: f32) {
        self.0.step(dt);
    }

    #[inline]
    #[getter(car)]
    fn get_car(&self) -> Car {
        self.0.car.into()
    }

    #[inline]
    #[setter(car)]
    fn set_car(&mut self, car: Car) {
        self.0.car = car.0;
    }

    #[inline]
    #[getter(speed)]
    fn get_speed(&self) -> f32 {
        self.0.speed
    }

    #[inline]
    #[setter(speed)]
    fn set_speed(&mut self, speed: f32) {
        self.0.speed = speed;
    }

    #[inline]
    #[getter(target)]
    fn get_target(&self) -> Vec3 {
        self.0.target.into()
    }

    #[inline]
    #[setter(target)]
    fn set_target(&mut self, target: Vec3) {
        self.0.target = target.into();
    }

    #[inline]
    #[getter(controls)]
    fn get_controls(&self) -> Input {
        self.0.controls.into()
    }

    #[inline]
    #[getter(finished)]
    fn get_finished(&self) -> bool {
        self.0.finished
    }

    #[inline]
    fn __str__(&self) -> String {
        format!("{self:?}")
    }
}

pynamedmodule! {
    doc: "",
    name: mechanics,
    funcs: [],
    classes: [Drive],
    submodules: []
}

pynamedmodule! {
    doc: "",
    name: linear_algebra,
    funcs: [],
    classes: [Vec3],
    submodules: []
}

pynamedmodule! {
    doc: "",
    name: simulation,
    funcs: [],
    classes: [Game, Ball, Field, Input],
    submodules: []
}

pynamedmodule! {
    doc: "RLUtilities bindings for Python 3.7+",
    name: rlutilities,
    funcs: [initialize],
    classes: [],
    submodules: [simulation, linear_algebra, mechanics]
}

#[pyfunction]
#[inline]
fn initialize(asset_dir: String) {
    rlu::initialize(asset_dir);
}
