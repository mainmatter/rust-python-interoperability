# Constructors

In the previous section (and its exercise) we relied on a `#[pyfunction]` as the constructor for the `#[pyclass]`
we defined. Without `new_wallet`, we wouldn't have been able to create new `Wallet` instances from Python.\
Let's now explore how to define a constructor directly within the `#[pyclass]` itself.

## Defining a constructor

You can add a constructor to your `#[pyclass]` using the `#[new]` attribute on a method. Here's an example:

```rust
use pyo3::prelude::*;

#[pyclass]
struct Wallet {
    #[pyo3(get, set)]
    balance: i32,
}

#[pymethods]
impl Wallet {
    #[new]
    fn new(balance: i32) -> Self {
        Wallet { balance }
    }
}
```

A Rust method annotated with `#[new]` is equivalent to the `__new__` method in Python. At the moment there is no way to
define the `__init__` method in Rust.\
The `impl` block containing the constructor must also be annotated with the `#[pymethods]` attribute for `#[new]`
to work as expected.

## Signature

Everything we learned about arguments in the context of `#[pyfunction]`s applies to constructors as well.\
In terms of output type, you can return `Self` if the constructor is infallible, or `PyResult<Self>` if it can fail.
