[![Project](https://img.shields.io/badge/Project-Holochain-blue.svg?style=flat-square)](http://holochain.org/)
[![Discord](https://img.shields.io/badge/Discord-DEV.HC-blue.svg?style=flat-square)](https://discord.gg/k55DS5dmPH)
[![License: CAL 1.0](https://img.shields.io/badge/License-CAL%201.0-blue.svg)](https://github.com/holochain/cryptographic-autonomy-license)
[![Twitter Follow](https://img.shields.io/twitter/follow/holochain.svg?style=social&label=Follow)](https://twitter.com/holochain)

# Holochain Serialization - Python

This project was generated using `maturin`, following the instructions given by [`Pyo3`](https://github.com/PyO3/pyo3).

### Set up a development environment

The developer environment for this project relies on Holonix, which you can find out more about in the Holochain [getting started guide](https://developer.holochain.org/get-started/). Once you have Nix installed, you can create a new development environment by entering the following command into your shell at the root of this project:

```bash
nix develop
```

Then once the Nix shell has spawned, create a virtual environment and install dependencies:

```bash
python -m venv .venv
source .venv/bin/activate
pip install .
```

### Verify changes

You can compile this Python module and the Rust code in one step with:

```bash
maturin develop
```

This will end up placing the module in your `.venv`'s site_packages (somethign like `.venv/lib/python3.11/site-packages/holochain_serialization`). You can then run the test script to verify that you haven't broken anything with:

```bash
python test.py
```

You may of course have broken that check because if you change the input in any way, you will change the hashes that being output and checked against the currently expected values.
