# Static methods

All the class methods we've seen so far have been **instance methods**—i.e. they take an instance of the class
as one of their arguments.\
Python supports **static methods** as well. These methods don't take an instance of the class as an argument,
but they are "attached" to the class itself.

The same concept exists in Rust:

```rust
pub struct Wallet {
    balance: i32,
}

impl Wallet {
    pub fn default() -> Self {
        Wallet { balance: 0 }
    }
}
```

`Wallet::get_default` is a static method since it doesn't take `self` or references to `self` as arguments.\
You might then expect the following to define a Python static method on the `Wallet` class:

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

    fn default() -> Self {
        Wallet { balance: 0 }
    }
}
```

However, this code will not compile.\
To define a static method in Python, you need to explicitly mark it with the `#[staticmethod]` attribute:

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
 
    // Notice the `#[staticmethod]` attribute here!
    #[staticmethod]
    fn default() -> Self {
        Wallet { balance: 0 }
    }
}
```

## Class methods

Python also supports **class methods**. These methods take the class itself as an argument, rather than an instance of the class.\
In Rust, you can define class methods by taking `cls: &PyType` as the first argument:

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

    // Notice the `cls` argument here!
    #[classmethod]
    fn from_str(_cls: &Bound<'_, PyType>, balance: &str) -> PyResult<Self> {
        let balance = balance.parse::<i32>()?;
        Ok(Wallet { balance })
    }
}
```

Since you can directly refer to the class in a Rust static method (i.e. the `Self` type), you won't find yourself
using class methods as often as you would in Python.
