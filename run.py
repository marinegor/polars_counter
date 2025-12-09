import polars as pl
from polars_counter import pig_latinnify, plus_one


df = pl.DataFrame(
    {
        "english": ["this", "is", "not", "pig", "latin"],
        "number": [1, 2, 3, 4, 5],
    }
)
result = df.with_columns(
    pig_latin=pig_latinnify("english"),
    pls1=plus_one("number"),
)
print(result)
