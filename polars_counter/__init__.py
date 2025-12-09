from __future__ import annotations

from pathlib import Path
from typing import TYPE_CHECKING

import polars as pl
from polars.plugins import register_plugin_function

from polars_counter._internal import __version__ as __version__

if TYPE_CHECKING:
    from polars_counter.typing import IntoExprColumn

LIB = Path(__file__).parent


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
