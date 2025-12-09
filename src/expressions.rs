#![allow(clippy::unused_unit)]
use std::fmt::Write;

use bincode::{deserialize, serialize};
use polars::prelude::*;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyBytesMethods, PyInt};
use pyo3_polars::derive::polars_expr;
use serde::{Deserialize, Serialize};

use crate::impl_pickle;

// #[derive(Deserialize, Serialize, Default, Clone, PartialEq, Eq, FromPyObject)]
#[derive(Serialize, Deserialize, Clone, Debug)]
#[pyclass(name = "PyCounter", module = "polars_counter")]
pub struct Counter {
    cnt: i64,
}

#[pymethods]
impl Counter {
    fn emit(&mut self) -> PyResult<i64> {
        eprintln!("emit(), self= {:?}", self);
        Ok(self._emit())
    }

    #[new]
    pub fn new(value: i64) -> PyResult<Self> {
        eprintln!("__new__, value= {}", value);
        Ok(Counter { cnt: value })
    }

    fn __getnewargs__(&self) -> PyResult<(i64,)> {
        eprintln!("__getnewargs__, self= {:?}", self);
        Ok((self.cnt,))
    }

    //fn __getstate__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyBytes>> {
    //    // Used in pickle/pickling
    //    Ok(PyBytes::new(
    //        py,
    //        &py.enter_polars(|| self.series.read().serialize_to_bytes())?,
    //    ))
    //}

    //fn __setstate__(&self, py: Python<'_>, state: Py<PyAny>) -> PyResult<()> {
    //    // Used in pickle/pickling
    //    use pyo3::pybacked::PyBackedBytes;
    //    match state.extract::<PyBackedBytes>(py) {
    //        Ok(bytes) => py.enter_polars(|| {
    //            let mut reader = std::io::Cursor::new(&*bytes);
    //            *self.series.write() = Series::deserialize_from_reader(&mut reader)?;
    //            PolarsResult::Ok(())
    //        }),
    //        Err(e) => Err(e),
    //    }
    //}
}

impl_pickle!(Counter);

impl Counter {
    fn _emit(&mut self) -> i64 {
        let rv = self.cnt + 1;
        self._consume(1);
        rv
    }

    fn _consume(&mut self, num: i64) {
        self.cnt += num;
    }

    fn from(bytes: Vec<u8>) -> Self {
        let array: [u8; 8] = bytes
            .try_into()
            .map_err(|_| "Failed to convert vector into array")
            .unwrap();
        Counter {
            cnt: i64::from_ne_bytes(array),
        }
    }
}

#[derive(Deserialize, FromPyObject)]
struct PlusNKwargs {
    n: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[pyclass(name = "PlusCounterKwargs", module = "polars_counter")]
struct PlusCounterKwargs {
    counter: Counter,
}

impl_pickle!(PlusCounterKwargs);

#[polars_expr(output_type=Int64)]
pub fn plus_counter(inputs: &[Series], mut kwargs: PlusCounterKwargs) -> PolarsResult<Series> {
    let ca = inputs[0].i64().expect("could not create chunked array");
    let num = kwargs.counter.emit().unwrap();
    let out: Int64Chunked = ca.apply(|opt_v: Option<i64>| opt_v.map(|v: i64| v + num));
    Ok(out.into_series())
}

#[polars_expr(output_type=Int64)]
pub fn plus_n(inputs: &[Series], kwargs: PlusNKwargs) -> PolarsResult<Series> {
    let ca = inputs[0].i64().expect("could not create chunked array");
    let out: Int64Chunked = ca.apply(|opt_v: Option<i64>| opt_v.map(|v: i64| v + kwargs.n));
    Ok(out.into_series())
}

#[polars_expr(output_type=Int64)]
pub fn plus_one(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].i64().expect("could not create chunked array");
    let out: Int64Chunked = ca.apply(|opt_v: Option<i64>| opt_v.map(|v: i64| v + 1));
    Ok(out.into_series())
}

#[polars_expr(output_type=String)]
fn pig_latinnify(inputs: &[Series]) -> PolarsResult<Series> {
    let ca: &StringChunked = inputs[0].str()?;
    let out: StringChunked = ca.apply_into_string_amortized(|value: &str, output: &mut String| {
        if let Some(first_char) = value.chars().next() {
            write!(output, "{}{}ay", &value[1..], first_char).unwrap()
        }
    });
    Ok(out.into_series())
}
