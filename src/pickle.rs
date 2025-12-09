/// macro to implement the necessary methods to pickle and unpickle a python
/// bound object in pyo3.
///
///
/// Rust structs bound to python are not able to automatically pickle and unpickle
/// because pyo3 does not implement the `__getstate__` and `__setstate__` methods
/// for them. This macro implements those methods for the given struct.
/// See https://github.com/PyO3/pyo3/issues/100 for more information.
///
/// For this struct to be applied to a struct, the struct must implement the
/// `Serialize` and `Deserialize` traits from serde. The format is an internal
/// implementation detail and is subject to change.
///
#[macro_export]
macro_rules! impl_pickle {
    ($struct_name:ident) => {
        #[pymethods]
        impl $struct_name {
            pub fn __setstate__(
                &mut self,
                state: &pyo3::Bound<'_, pyo3::types::PyBytes>,
            ) -> pyo3::PyResult<()> {
                eprintln!("__setstate__, self={:?}", self);
                *self = rmp_serde::from_slice(state.as_bytes())
                    .map_err(|e| $crate::errors::CounterError {
                        message: (format!(
                            "Failed to unpickle {}: {}",
                            stringify!($struct_name),
                            e.to_string()
                        )),
                    })
                    .unwrap();
                eprintln!("  state.as_bytes(): {:?}", state.as_bytes());
                Ok(())
            }

            pub fn __getstate__<'py>(
                &self,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::types::PyBytes>> {
                let state = rmp_serde::to_vec(&self)
                    .map_err(|e| $crate::errors::CounterError {
                        message: (format!(
                            "Failed to unpickle {}: {}",
                            stringify!($struct_name),
                            e.to_string()
                        )),
                    })
                    .ok();
                let bytes = pyo3::types::PyBytes::new(py, &state.unwrap());
                eprintln!("__getstate__, self={:?}", self);
                eprintln!("  bytes: {:?}", bytes);
                Ok(bytes)
            }
        }
    };
}

//    fn __getstate__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
//        // Used in pickle/pickling
//        Ok(PyBytes::new(
//            py,
//            &py.enter_polars(|| self.series.read().serialize_to_bytes())?,
//        ))
//    }
//
//    fn __setstate__(&self, py: Python<'_>, state: Py<PyAny>) -> PyResult<()> {
//        // Used in pickle/pickling
//        use pyo3::pybacked::PyBackedBytes;
//        match state.extract::<PyBackedBytes>(py) {
//            Ok(bytes) => py.enter_polars(|| {
//                let mut reader = std::io::Cursor::new(&*bytes);
//                *self.series.write() = Series::deserialize_from_reader(&mut reader)?;
//                PolarsResult::Ok(())
//            }),
//            Err(e) => Err(e),
//        }
//    }
