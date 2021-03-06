#  Copyright (c) 2021. Davi Pereira dos Santos
#  This file is part of the hosh project.
#  Please respect the license - more about this in the section (*) below.
#
#  hosh is free software: you can redistribute it and/or modify
#  it under the terms of the GNU General Public License as published by
#  the Free Software Foundation, either version 3 of the License, or
#  (at your option) any later version.
#
#  hosh is distributed in the hope that it will be useful,
#  but WITHOUT ANY WARRANTY; without even the implied warranty of
#  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
#  GNU General Public License for more details.
#
#  You should have received a copy of the GNU General Public License
#  along with hosh.  If not, see <http://www.gnu.org/licenses/>.
#
#  (*) Removing authorship by any means, e.g. by distribution of derived
#  works or verbatim, obfuscated, compiled or rewritten versions of any
#  part of this work is a crime and is unethical regarding the effort and
#  time spent here.
#  Relevant employers or funding agencies will be notified accordingly.

# __version__ = '0.2103.1'
from .colors import colorize128bit
# noinspection PyUnresolvedReferences,PyPackageRequirements
from .hoshrust import (
    n_bin_id_fromblob, n_id_fromperm, bin_id_fromn, n_bin_fromid, mul, minv, div, muls, mulpairs
)


class Hash:
    _inv, _repr = None, None

    def __init__(self, n=None, blob=None, id=None, bin=None):
        if blob:  # 1.54us
            self._n, self._bin, self._id = n_bin_id_fromblob(blob)
        else:
            self._n, self._bin, self._id = n, bin, id

    def calculate(self):
        if self._bin:  # 1.11us
            self._n, self._id = n_id_fromperm(self._bin)
        elif self._id:  # 1.17us
            self._n, self._bin = n_bin_fromid(self._id)
        elif self._n is not None:  # 1.49us vs 3.7us (rust vs python)
            self._bin, self._id = bin_id_fromn(self._n)
        else:
            raise Exception("Missing argument.")

    @property
    def bin(self):
        if self._bin is None:
            self.calculate()
        return self._bin

    @property
    def id(self):
        if self._id is None:
            self.calculate()
        return self._id

    @property
    def n(self):
        if self._n is None:
            self.calculate()
        return self._n

    def __mul__(self, other):
        return Hash(bin=mul(self.bin, other.bin))  # 1.43us vs 2.47us

    def __invert__(self):  # 560ns (rust)
        if self._inv is None:
            self._inv = Hash(bin=minv(self.bin))
        return self._inv

    def __truediv__(self, other):  # 704ns vs 1.26us (div vs mul+minv)
        return Hash(bin=div(self.bin, other.bin))

    def __add__(self, other):  # 660ns vs 534ns (better with python)
        return Hash(n=(self.n + other.n) % 295232799039604140847618609643520000000)  # 34!

    def __sub__(self, other):  # 530ns (python)
        return Hash(n=(self.n - other.n) % 295232799039604140847618609643520000000)  # 34!

    def __str__(self):
        return self.id

    def __repr__(self):
        if self._repr is None:
            self._repr = colorize128bit(self.n, self.id)
        return self._repr

    @classmethod
    def muls(cls, *perms):
        # pp = (p.bin for p in perms)
        # bytes = pack('34s' * len(perms), *pp)
        # return Halg(bin=mulmany_ser(bytes))
        return Hash(bin=muls([p.bin for p in perms]))  # faster than pack at least until 15 elements (2.66us)

    @classmethod
    def pairmuls(cls, *pairs):
        results = mulpairs([(p.bin) for p in pairs])
        return [Hash(bin=res) for res in results]

    def __hash__(self):
        return self.n

    def __eq__(self, other):
        return self.n == other.n

"""
benchmark

In [1]: import timeit
   ...: setup = '''
   ...: from hoshrust import Hash
   ...: a= Hash(9374713407134097134097314079071324970)
   ...: b= Hash(blob=b"9374713407134097134097345364356435346")
   ...: c= Hash(9864713407134097134097314079071324970)
   ...: d= Hash(blob=b"9374713407134097134097314079071324dsf0")
   ...: '''
   ...: n=100000
   ...: r=25
   ...: u = round(10*min(timeit.repeat('Hash(9374713407134097134097314079071324970).bin', number=n, repeat=r, setup=setup)),2)
   ...: d = round(10*min(timeit.repeat('~Hash(blob=b"9374713407134097134097345364356435346")', number=n, repeat=r, setup=setup)),2)
   ...: t = round(10*min(timeit.repeat('Hash(9864713407134097134097314079071324970).id', number=n, repeat=r, setup=setup)),2)
   ...: q = round(10*min(timeit.repeat('Hash(blob=b"9374713407134097134097314079071324dsf0").n', number=n, repeat=r, setup=setup)),2)
   ...: c = round(10*min(timeit.repeat('a*b*c*d', number=n, repeat=r, setup=setup)),2)
   ...: s = round(10*min(timeit.repeat('Hash(blob=b"9374713407134097134097345364356435346")', number=n, repeat=r, setup=setup)),2)
   ...: print(u,d,t,q,c,s,"us")


[0.9470198675496688,
 0.15437262357414447,
 0.1764705882352941,
 0.11887550200803214,
 0.3278443113772455,
 0.12307692307692308]

In [13]: list(map(lambda x,y:float(x)/float(y), "1.96 2.58 1.97 1.92 2.25 1.97".split(" "), "1.51 13.15 8.33 12.45 6.68 11.05".split(" ")))
Out[13]: 
[1.2980132450331126,
 0.1961977186311787,
 0.23649459783913565,
 0.15421686746987953,
 0.33682634730538924,
 0.17828054298642532]


"""
