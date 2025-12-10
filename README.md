# polars_counter


Install:

```bash
uv sync
```

Run python testing code:

```
uv run run.py
```

It'll produce multiple pickle files, and then you can run:


```
cargo build

for f in *.pickle; do
    cargo run $f
done
```