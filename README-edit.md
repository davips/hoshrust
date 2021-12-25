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
<<operation>>




### Features
