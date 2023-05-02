use crate::{cmat3, cvec3};
use pyo3::{exceptions::PyIndexError, prelude::*, pyclass::CompareOp, types::PyTuple};

#[pyclass]
#[derive(Clone, Copy, PartialEq, PartialOrd, Default, Debug)]
#[pyo3(name = "mat3")]
pub struct Mat3([f32; 9]);

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
pub struct Vec3([f32; 3]);

impl From<[f32; 3]> for Vec3 {
    #[inline]
    fn from(value: [f32; 3]) -> Self {
        Self(value)
    }
}

impl From<Vec3> for [f32; 3] {
    #[inline]
    fn from(value: Vec3) -> Self {
        value.0
    }
}

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
    pub fn __repr__(&self) -> String {
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
