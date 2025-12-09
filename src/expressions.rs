#![allow(clippy::unused_unit)]
use std::fmt::Write;

use polars::prelude::*;
use pyo3::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;

#[derive(Deserialize)]
#[pyclass]
pub struct Counter {
    cnt: i64,
}

#[pymethods]
impl Counter {
    #[new]
    fn new(value: i64) -> Self {
        Counter { cnt: value }
    }

    fn emit(&mut self) -> PyResult<i64> {
        Ok(self._emit())
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

#[derive(Deserialize)]
struct PlusNKwargs {
    n: i64,
}

#[derive(Deserialize)]
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
