# Classes

We've covered Python functions written in Rust, but what about classes?

## Defining a class

You can use the `#[pyclass]` attribute to define a new Python class in Rust. Here's an example:

```rust
use pyo3::prelude::*;

#[pyclass]
struct Wallet {
    balance: i32,
}
```

It defines a new Python class called `Wallet` with a single field, `balance`.

## Registering a class

Just like with `#[pyfunction]`s, you must explicitly register your class with a module to make it visible to
users of your extension.\
Continuing with the example above, you'd register the `Wallet` class like this:

```rust
#[pymodule]
fn my_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Wallet>()?;
    Ok(())
}
```

## `IntoPyObject`

Rust types that have been annotated with `#[pyclass]` automatically implement the `IntoPyObject` trait, thus
allowing you to return them from your `#[pyfunction]`s.

For example, you can define a function that creates a new `Wallet` instance:

```rust
#[pyfunction]
fn new_wallet(balance: i32) -> Wallet {
    Wallet { balance }
}
```

It'll compile just fine, handing over a new `Wallet` instance to the Python caller.

## Attributes

By default, the fields of your `#[pyclass]`-annotated structs aren't accessible to Python callers.\
Going back to our `Wallet` example—if you try to access the `balance` field from Python, you'll get an error:

```text
        wallet = new_wallet(0)
>       assert wallet.balance == 0
E       AttributeError: 'builtins.Wallet' object has no attribute 'balance'

tests/test_sample.py:8: AttributeError
```

The same error would occur even if you made `balance` a public field.

To make the field accessible to Python, you must add a **getter**.\
This can be done using the `#[pyo3(get)]` attribute:

```rust
#[pyclass]
struct Wallet {
    #[pyo3(get)]
    balance: i32,
}
```

Now, the `balance` field is accessible from Python:

```python
def test_wallet():
    wallet = new_wallet(0)
    assert wallet.balance == 0
```

If you want to allow Python callers to modify the field, you can add a **setter** using the `#[pyo3(set)]` attribute:

```rust
#[pyclass]
struct Wallet {
    // Both getter and setter
    #[pyo3(get, set)]
    balance: i32,
}
```
