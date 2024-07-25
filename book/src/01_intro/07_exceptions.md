# Exceptions

Python and Rust have different error handling mechanisms.\
In Python, you raise exceptions to signal that something went wrong.\
In Rust, errors are normal values that you return from functions, usually via the `Result` type.

`pyo3` provides `PyResult<T>` to help you bridge the gap between these two worlds.

## `PyResult<T>`

`PyResult<T>` is the type you'll return whenever your `#[pyfunction]` can fail.\
It is a type alias for `Result<T, PyErr>`, where `PyErr` is `pyo3`'s representation of a Python exception.
`pyo3` will automatically raise a Python exception whenever a `#[pyfunction]` returns `Err(PyErr)` value:

```rust
use pyo3::prelude::*;
use pyo3::types::PyAny;

#[pyfunction]
fn print_if_number(item: Bound<'_, PyAny>) -> PyResult<()> {
    let number = item.extract::<u64>()?;
    println!("{}", number);
    Ok(())
}
```

In the example above, `extract::<u64>()?` returns a `PyResult<u64>`.\
If the object is not an unsigned integer, `extract` will return an error, which will be propagated up to the caller
via the `?` operator. On the Python side, this error will be raised as a Python exception by `pyo3`.

## Built-in exception types

You should be intentional about the types of exceptions you raise. What kind of error are you signaling?
What is the caller expected to catch?

All built-in Python exceptions are available in `pyo3::exceptions`—e.g. `pyo3::exceptions::PyValueError` for
a [`ValueError`](https://docs.python.org/3/library/exceptions.html#ValueError). You can use their `new_err`
method to create an instance.

## Panics

Rust provides another mechanism for handling "unrecoverable" errors: panics. What happens if you panic in a `#[pyfunction]`?\
`pyo3` will catch the panic and raise a `pyo3_runtime.PanicException` to the Python caller. You've probably seen this
behaviour at play when solving the exercises associated to the previous sections.
