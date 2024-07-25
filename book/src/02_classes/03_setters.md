# Custom setters and getters

In [a previous section](01_constructors.md), we learned how to attach the default getter and setter to a field in a `#[pyclass]`:

```rust
use pyo3::prelude::*;

#[pyclass]
struct Wallet {
    #[pyo3(get, set)]
    balance: i32,
}
```

This is convenient, but it's not always desirable!\
Let's introduce an additional constraint to our `Wallet` struct: the balance should never go below a pre-determined
overdraft threshold.
We'd start by enforcing this constraint in the constructor method:

```rust
use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

#[pyclass]
struct Wallet {
    #[pyo3(get, set)]
    balance: i32,
}

const OVERDRAFT_LIMIT: i32 = -100;

#[pymethods]
impl Wallet {
    #[new]
    fn new(balance: i32) -> PyResult<Self> {
        if balance < OVERDRAFT_LIMIT {
           return Err(PyValueError::new_err("Balance cannot be below overdraft limit"));     
        }
        Ok(Wallet { balance })
    }
}
```

`Wallet::new` ensures that a newly-created `Wallet` upholds the overdraft constraint. But the default setter
can be easily used to circumvent the limit:

```python
wallet = Wallet(0)
wallet.balance = -200 # This should not be allowed, but it is!
```

## `#[setter]` and `#[getter]`

We can override the default getter and setter by defining custom methods for them.\
Here's how we can implement a custom setter for the `balance` field via the `#[setter]` attribute:

```rust
use pyo3::prelude::*;

#[pyclass]
struct Wallet {
    // We keep using the default getter, no issues there
    #[pyo3(get)]
    balance: i32,
}

const OVERDRAFT_LIMIT: i32 = -100;

#[pymethods]
impl Wallet {
    #[new]
    fn new(balance: i32) -> PyResult<Self> {
        Wallet::check_balance(balance)?;
        Ok(Wallet { balance })
    }

    #[setter]
    fn set_balance(&mut self, value: i32) {
        Wallet::check_balance(value)?;
        self.balance = value;
    }
}

impl Wallet {
    // We put this method in a separate `impl` block to avoid exposing it to Python
    fn check_balance(balance: i32) -> PyResult<()> {
        if balance < OVERDRAFT_LIMIT {
            return Err(PyValueError::new_err("Balance cannot be below overdraft limit"));
        }
        Ok(())
    }
}
```

Every time the `balance` field is set in Python, `Wallet::set_balance` will be called:

```python
wallet = Wallet(0)
wallet.balance = -200  # Now raises a `ValueError`
```

The field is associated with its setter using a conventional naming strategy for the setter method: `set_<field_name>`.
You can also explicitly specify the field name in the `#[setter]` attribute, like this: `#[setter(balance)]`.

Custom getters are defined in a similar way using the `#[getter]` attribute. The naming convention for
getter methods is `<field_name>`, but you can also specify the field name explicitly in the attribute—e.g.
`#[getter(balance)]`.
