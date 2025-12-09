import pickle

import polars as pl

from polars_counter import Counter, pig_latinnify, plus_counter, plus_n, plus_one

print("cnt = Counter(0)")
cnt = Counter(0)
print("-" * 20)

print(f"{cnt.emit()=}")
print("-" * 20)

print("dumps = pickle.dumps(cnt)")
dumps = pickle.dumps(cnt)
print("cnt_re = pickle.loads(dumps)")
cnt_re = pickle.loads(dumps)
print("-" * 20)

print(f"{cnt.emit()=}")
print(f"{cnt_re.emit()=}")


df = pl.DataFrame(
    {
        "english": ["this", "is", "not", "pig", "latin"],
        "number": pl.Series([1, 2, 3, 4, 5]),
    }
)
result = df.with_columns(
    pig_latin=pig_latinnify("english"),
    pls1=plus_one("number"),
    pls3=plus_n("number", n=3),
    emit=plus_counter("number", counter=cnt),
)
print(result)
