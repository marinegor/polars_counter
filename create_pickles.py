import pickle

import polars as pl

from polars_counter import Counter

print("==" * 20)
print("= " * 10, "Starting run", " =" * 20)
print("==" * 20)

print("cnt = Counter(0)")
cnt = Counter(0)
print("-" * 20)

print(f"{cnt.emit()=}")
print("-" * 20)

print("dumps = pickle.dumps(cnt)")
dumps = pickle.dumps(cnt, protocol=5)
print("cnt_re = pickle.loads(dumps)")
cnt_re = pickle.loads(dumps)
print("-" * 20)

print(f"{cnt.emit()=}")
print(f"{cnt_re.emit()=}")


with open("py_counter.pickle", "wb") as fout:
    pickle.dump(cnt, fout, protocol=5)
with open("rust_counter.pickle", "wb") as fout:
    pickle.dump(cnt._pyc, fout, protocol=5)
with open("tuple.pickle", "wb") as fout:
    pickle.dump((1, 2, 3), fout, protocol=5)
with open("dict.pickle", "wb") as fout:
    pickle.dump({1: "foo", 2: "bar", "baz": 3}, fout, protocol=5)
