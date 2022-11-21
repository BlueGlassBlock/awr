use crate::utils::py_obj;
/// implements FakeDispatcher
use pyo3::prelude::*;

#[pyclass(subclass)]
pub struct FakeDispatcher;

#[pymethods]
impl FakeDispatcher {
    /// Returns an Awaitable[None] to fake a dispatcher.
    pub fn catch<'py>(&self, py: Python<'py>) -> PyResult<&'py PyAny> {
        pyo3_asyncio::tokio::future_into_py(py, async { Ok(Python::with_gil(|py| py.None())) })
    }
}

#[pyclass(subclass)]
pub struct EventBase;

#[pymethods]
impl EventBase {
    #[new]
    fn new() -> Self {
        Self {}
    }

    #[getter]
    #[allow(non_snake_case)]
    pub fn Dispatcher(&self) -> PyResult<Py<FakeDispatcher>> {
        py_obj(FakeDispatcher {})
    }
}
