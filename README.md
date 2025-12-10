# polars_counter


Install:

```bash
uv sync
```

Create pickle files:

```
uv run create_pickles.py
```

It'll produce multiple pickle files, and then you can run:


```
cargo build

for f in *.pickle; do
    cargo run $f
done
```

If you wanna check polars expr, run

```
uv run run_polars_expr.py
```
