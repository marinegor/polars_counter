import pickle
import polars as pl
from polars_counter import pig_latinnify, plus_one, plus_n, Counter, plus_counter
import pickle


cnt = Counter(0)
print(f"{cnt.emit()=}")
cnt_re = pickle.loads(pickle.dumps(cnt))
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
