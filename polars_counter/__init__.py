from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

import polars as pl
from polars.plugins import register_plugin_function
import json

from polars_counter._internal import __version__ as __version__, PyCounter

if TYPE_CHECKING:
    from polars_counter.typing import IntoExprColumn, IntoExpr

LIB = Path(__file__).parent


class Counter:
    _pyc: PyCounter = None

    def __new__(cls, *args, **kwargs):
        print(f"__new__, {args=} {kwargs=}")
        instance = super(Counter, cls).__new__(cls)
        return instance

    def __init__(self, value: int = 0):
        print(f"__init__, {self=}")
        self._pyc = PyCounter(value)

    def __getstate__(self) -> bytes:
        print(f"__getstate__, {self=}")
        rv = self._pyc.__getstate__()
        print(f"  {rv=}, {type(rv)=}")
        return rv

    def __setstate__(self, state: bytes) -> None:
        print(f"__setstate__, {self=}")
        self._pyc = Counter()._pyc  # Initialize with a dummy
        self._pyc.__setstate__(state)

    @classmethod
    def _from_pyc(cls, _pyc: PyCounter) -> Counter:
        pyc = cls.__new__(cls)
        pyc._pyc = _pyc
        return pyc

    def emit(self) -> int:
        print(f"emit(self), {self=}")
        return self._pyc.emit()


def pig_latinnify(expr: IntoExprColumn) -> pl.Expr:
    return register_plugin_function(
        args=[expr],
        plugin_path=LIB,
        function_name="pig_latinnify",
        is_elementwise=True,
    )


def plus_one(expr: IntoExprColumn) -> pl.Expr:
    return register_plugin_function(
        args=[expr],
        plugin_path=LIB,
        function_name="plus_one",
        is_elementwise=True,
    )


def plus_n(expr: IntoExprColumn, *, n: int) -> pl.Expr:
    return register_plugin_function(
        args=[expr],
        plugin_path=LIB,
        function_name="plus_n",
        is_elementwise=True,
        kwargs={"n": n},
    )


def plus_counter(expr: IntoExpr, *, counter: Counter) -> pl.Expr:
    return register_plugin_function(
        args=expr,
        plugin_path=LIB,
        function_name="plus_counter",
        is_elementwise=True,
        kwargs={"counter": counter},
    )
