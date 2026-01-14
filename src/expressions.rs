#![allow(clippy::unused_unit)]
use std::fmt::Write;

use polars::prelude::*;
use pyo3::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::{Deserialize, Serialize};

mod errors {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CounterError {
        pub message: String,
    }
}

// #[derive(Deserialize, Serialize, Default, Clone, PartialEq, Eq, FromPyObject)]
#[derive(Clone, Debug)]
#[pyclass(name = "PyCounter", module = "polars_counter")]
pub struct Counter {
    cnt: i64,
}

impl serde::Serialize for Counter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let bytes = bincode::serialize(&self.cnt).map_err(serde::ser::Error::custom)?;
        serializer.serialize_bytes(&bytes)
    }
}

impl<'de> serde::Deserialize<'de> for Counter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FieldVisitor;

        impl<'de> serde::de::Visitor<'de> for FieldVisitor {
            type Value = Counter;
            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str(concat!(
                    "a byte array containing bincode-serialized ",
                    stringify!(MyClass),
                    " data"
                ))
            }
            fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                let cnt: i64 = bincode::deserialize(v).map_err(serde::de::Error::custom)?;
                Ok(Counter { cnt })
            }
            fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                self.visit_bytes(&v)
            }
        }
        deserializer.deserialize_bytes(FieldVisitor)
    }
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

    pub fn __setstate__(
        &mut self,
        state: &pyo3::Bound<'_, pyo3::types::PyBytes>,
    ) -> pyo3::PyResult<()> {
        self.cnt = bincode::deserialize(state.as_bytes()).unwrap();
        Ok(())
    }

    pub fn __getstate__<'py>(
        &self,
        py: pyo3::Python<'py>,
    ) -> pyo3::PyResult<pyo3::Bound<'py, pyo3::types::PyBytes>> {
        Ok(pyo3::types::PyBytes::new(
            py,
            &bincode::serialize(&self.cnt).unwrap(),
        ))
    }
    pub fn __getnewargs__(&self) -> pyo3::PyResult<(i64,)> {
        Ok((self.cnt.clone(),))
    }
}

impl Counter {
    fn _emit(&mut self) -> i64 {
        let rv = self.cnt + 1;
        self._consume(1);
        rv
    }

    fn _consume(&mut self, num: i64) {
        self.cnt += num;
    }
}

#[derive(Deserialize, FromPyObject, Clone)]
struct PlusNKwargs {
    n: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[pyclass(name = "PlusCounterKwargs", module = "polars_counter")]
struct PlusCounterKwargs {
    counter: Counter,
}

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
