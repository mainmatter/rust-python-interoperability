# Methods

The `#[pymethods]` attribute is not limited to constructors. You can use it to attach any number of methods to your `#[pyclass]`:

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

    fn deposit(&mut self, amount: i32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: i32) {
        self.balance -= amount;
    }
}
```

All methods within an `impl` block annotated with `#[pymethods]` are automatically exposed to Python as methods on
your `#[pyclass]`[^visibility]. The `deposit` and `withdraw` methods in the example above can be called from Python like this:

```python
wallet = Wallet(0)
wallet.deposit(100)
wallet.withdraw(50)
assert wallet.balance == 50
```

## `multiple-pymethods`

You can't annotate multiple `impl` blocks with `#[pymethods]` for the same class, due to a limitation in
Rust's metaprogramming capabilities.\
There is a way to work around this issue using some linker dark magic, via the
`multiple-pymethods` feature flag, but it comes with a penalty in terms of compile times as well as limited cross-platform support.
Check out [`pyo3`'s documentation](https://pyo3.rs/v0.23.3/class#implementation-details) for more details.

## Footnotes

[^visibility]: All methods in a `#[pymethods]` block are exposed, even if they are private!
