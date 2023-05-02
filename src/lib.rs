mod ball;
mod base;
mod car;
mod drive;
mod field;
mod game;
mod input;
mod math;
mod pytypes;

pub use ball::Ball;
pub use car::Car;
pub use drive::Drive;
pub use field::Field;
pub use game::Game;
pub use input::Input;
pub use math::{Mat3, Vec3};
pub use rlutilities_rs::{cmat3, cvec3, linear_algebra as linalg, mechanics as mech, rlu, simulation as sim};

use pyo3::{prelude::*, wrap_pyfunction, wrap_pymodule};
use pytypes::{FieldInfoPacket, GameTickPacket};

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
    classes: [Game, Ball, Field, Input, Car],
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
