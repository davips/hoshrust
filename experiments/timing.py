from garoupa import ø, Hosh as ho
from hoshrust import Hash as ha
haa = ha(blob=b"a")
hab = ha(blob=b"b")
hoa = ho(b"a")
hob = ho(b"b")
%timeit haa * hab
%timeit hoa * hob
print()
%timeit (ha(blob=b"a") * ha(blob=b"b")).id
%timeit (ho(b"a") * ho(b"b")).id
705 ns ± 7.55 ns per loop (mean ± std. dev. of 7 runs, 1000000 loops each)
2.17 µs ± 29.8 ns per loop (mean ± std. dev. of 7 runs, 100000 loops each)

4.98 µs ± 58.9 ns per loop (mean ± std. dev. of 7 runs, 100000 loops each)
35.9 µs ± 607 ns per loop (mean ± std. dev. of 7 runs, 10000 loops each)
