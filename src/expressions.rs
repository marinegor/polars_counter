#![allow(clippy::unused_unit)]
use std::fmt::Write;

use polars::chunked_array::ops::arity::unary_elementwise;
use polars::prelude::*;
use pyo3::prelude::*;
use pyo3_polars::derive::polars_expr;

#[pyclass]
struct Counter {
    cnt: u16,
}

#[pymethods]
impl Counter {
    #[new]
    fn new(value: u16) -> Self {
        Counter { cnt: value }
    }

    fn emit(&mut self) -> PyResult<u16> {
        Ok(self._emit())
    }
}

impl Counter {
    fn _emit(&mut self) -> u16 {
        self._consume(1);
        self.cnt + 1
    }

    fn _consume(&mut self, num: u16) {
        self.cnt += num;
    }
}

#[polars_expr(output_type=UInt16)]
pub fn plus_one(inputs: &[Series]) -> PolarsResult<Series> {
    let ca = inputs[0].i16()?;
    let out: Int16Chunked = ca.apply(|opt_v: Option<i16>| opt_v.map(|v: i16| v + 1));
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
