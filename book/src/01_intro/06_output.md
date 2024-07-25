# Output values

We've gone deep into the weeds of how `pyo3` handles arguments to your `#[pyfunction]`s.
Let's now move our focus to output values: how do you return _something_ from your Rust functions to Python?

## `IntoPy`

Guess what? There's a trait for that too!\
`IntoPy` is the counterpart of `FromPyObject`. It converts Rust values into Python objects:

```rust
pub trait IntoPy<T>: Sized {
    fn into_py(self, py: Python<'_>) -> T;
}
```

The output type of your `#[pyfunction]` must implement `IntoPy`.

### `IntoPy::into_py`

`IntoPy::into_py` expects two arguments:

- `self`: the Rust value you want to convert into a Python object.
- `Python<'_>`: a GIL token that you can use to create new Python objects.

## Case study: a newtype

Let's look at a simple example: a newtype that wraps a `u64`.
We want it to be represented as a "plain" integer in Python.

```rust
use pyo3::prelude::*;

struct MyType {
    value: u64,
}

impl IntoPy<Py<PyAny>> for MyType {
    fn into_py(self, py: Python<'_>) -> Py<PyAny> {
        self.value.to_object(py)
    }
}
```

## Provided implementations

`pyo3` provides out-of-the-box implementations of `IntoPy` for many Rust types, as well as for all `Py*` types.
Check out [its documentation](https://docs.rs/pyo3/0.22.0/pyo3/conversion/trait.IntoPy.html#) for an exhaustive list.
