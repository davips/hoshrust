![test](https://github.com/davips/hoshrust/workflows/test/badge.svg)
[![codecov](https://codecov.io/gh/davips/hoshrust/branch/main/graph/badge.svg)](https://codecov.io/gh/davips/hoshrust)

# hoshrust (see [hosh](https://github.com/davips/hosh) for the implementation of the current specification)
Fast cryptographic 22-digit hash and operators for Rust and Python.
This is based on the Symmetric group, which is not robust to many repetitions (see [hosh](https://github.com/davips/hosh) for a robust 40-digit version).
<p><a href="https://github.com/davips/hoshrust/blob/main/colored-id.png">
<img src="https://raw.githubusercontent.com/davips/hoshrust/main/colored-id.png" alt="Colored base-62 representation" width="500" height="130">
</a></p>

## Python installation
### from package
```bash
# Set up a virtualenv. 
python3 -m venv venv
source venv/bin/activate

# Install from PyPI
pip install hoshrust
```

### from source
```bash
cd my-project
git clone https://github.com/davips/hoshrust ../hoshrust
pip install -e ../hoshrust
```


### Examples
**Basic operations**
<details>
<p>

```python3
from hoshrust import Hash

# Hashes can be multiplied.
a = Hash(blob=b"Some large binary content...")
b = Hash(blob=b"Some other binary content. Might be, e.g., an action or another large content.")
c = a * b
print(f"{a} * {b} = {c}")
"""
0v58YxIhaae5NfYuXsoC1i * 04orKjYHAZraYORILOVwos = 3yT1A5oLlW2HpjSkgzo2yg
"""
```

```python3

# Multiplication can be reverted by the inverse hash. Zero is the identity hash.
print(f"{b} * {~b} = {b * ~b} = 0")
"""
04orKjYHAZraYORILOVwos * 211eErwhEiGnit0beo4tjo = 0000000000000000000000 = 0
"""
```

```python3

print(f"{c} * {~b} = {c * ~b} = {a} = a")
"""
3yT1A5oLlW2HpjSkgzo2yg * 211eErwhEiGnit0beo4tjo = 0v58YxIhaae5NfYuXsoC1i = 0v58YxIhaae5NfYuXsoC1i = a
"""
```

```python3

print(f"{~a} * {c} = {~a * c} = {b} = b")
"""
4q4X1jczNK2eKCV4uxEPNk * 3yT1A5oLlW2HpjSkgzo2yg = 04orKjYHAZraYORILOVwos = 04orKjYHAZraYORILOVwos = b
"""
```

```python3

# Division is shorthand for reversion.
print(f"{c} / {b} = {c / b} = a")
"""
3yT1A5oLlW2HpjSkgzo2yg / 04orKjYHAZraYORILOVwos = 0v58YxIhaae5NfYuXsoC1i = a
"""
```

```python3

# Hash multiplication is not expected to be commutative.
print(f"{a * b} != {b * a}")
"""
3yT1A5oLlW2HpjSkgzo2yg != 4AvOF9Fbhakd26mosfuuvR
"""
```

```python3

# Hash multiplication is associative.
print(f"{a * (b * c)} = {(a * b) * c}")
"""
51UdYbEAGI5mVogE4aFFKe = 51UdYbEAGI5mVogE4aFFKe
"""
```

```python3


```


</p>
</details>




### Features
