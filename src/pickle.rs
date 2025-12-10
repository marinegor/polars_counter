// the code below is modified from here: https://github.com/earth-mover/icechunk/blob/4bcf8065a65779d124556497d5245ee1db580ca4/icechunk-python/src/pickle.rs
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
            fn __setstate__(&mut self, py: Python<'_>, state: Py<PyAny>) -> pyo3::PyResult<()> {
                eprintln!("__setstate__, self={:?}", self);
                use pyo3::pybacked::PyBackedBytes;
                let bytes = state.extract::<PyBackedBytes>(py)?;
                *self = serde_pickle::from_slice(&bytes, serde_pickle::de::DeOptions::default())
                    .unwrap();
                Ok(())
            }

            fn __getstate__<'py>(
                &self,
                py: pyo3::Python<'py>,
            ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::types::PyBytes>> {
                let state = serde_pickle::to_vec(&self, serde_pickle::ser::SerOptions::default())
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
