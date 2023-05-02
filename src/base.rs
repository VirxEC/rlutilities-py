use pyo3::{prelude::*, PyClass};

#[inline]
pub fn repr_bool(b: bool) -> &'static str {
    if b {
        "True"
    } else {
        "False"
    }
}

pub trait PyDefault
where
    Self: Sized,
{
    fn py_default(py: Python) -> PyResult<Self>;
}

impl<T> PyDefault for T
where
    T: Default,
{
    fn py_default(_py: Python) -> PyResult<Self> {
        Ok(Self::default())
    }
}

pub trait FromGil<T>: Sized {
    fn from_gil(py: Python, obj: T) -> PyResult<Self>;
}

pub trait RemoveGil<T> {
    fn remove_gil(self, py: Python) -> T;
}

impl<T, U> RemoveGil<T> for Py<U>
where
    U: PyClass + Copy,
    T: From<U>,
{
    #[inline]
    fn remove_gil(self, py: Python) -> T {
        T::from(*self.borrow(py))
    }
}

#[macro_export]
macro_rules! new_gil {
    ($t:ty, $py:expr, $e:expr) => {
        Py::new($py, <$t>::from_gil($py, $e)?)?
    };
}

#[macro_export]
macro_rules! new_gil_default {
    ($t:ty, $py:expr) => {
        Py::new($py, <$t>::from_gil($py, <$t>::py_default($py)?)?)?
    };
}

pub trait IntoGil<T>: Sized {
    fn into_gil(self, py: Python) -> PyResult<T>;
}

impl<T, U> FromGil<U> for T
where
    T: From<U>,
{
    #[inline]
    fn from_gil(_py: Python, obj: U) -> PyResult<Self> {
        Ok(Self::from(obj))
    }
}

impl<T, U> IntoGil<U> for T
where
    U: FromGil<T>,
{
    #[inline]
    fn into_gil(self, py: Python) -> PyResult<U> {
        U::from_gil(py, self)
    }
}
