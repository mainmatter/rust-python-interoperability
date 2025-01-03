# Output values

We've gone deep into the weeds of how `pyo3` handles arguments to your `#[pyfunction]`s.
Let's now move our focus to output values: how do you return _something_ from your Rust functions to Python?

## `IntoPyObject`

Guess what? There's a trait for that too!\
`IntoPyObject` is the counterpart of `FromPyObject`. It converts Rust values into Python objects:

```rust
pub trait IntoPyObject<'py>: Sized {
    type Target;
    type Output: BoundObject<'py, Self::Target>;
    type Error: Into<PyErr>;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error>;
}
```

The output type of your `#[pyfunction]` must implement `IntoPyObject`.

### `IntoPyObject::into_pyobject`

`IntoPyObject::into_pyobject` expects two arguments:

- `self`: the Rust value you want to convert into a Python object.
- `Python<'py>`: a GIL token that you can use to create new Python objects.

The conversion can fail, so the method returns a `Result`.\
The output type itself is more complex, so let's break it down using an example.

## Case study: a newtype

Let's look at a simple example: a newtype that wraps a `u64`.
We want it to be represented as a "plain" integer in Python.

```rust
use std::convert::Infallible;
use pyo3::prelude::*;
use pyo3::types::PyInt;

struct MyType {
    value: u64,
}

impl<'py> IntoPyObject<'py> for MyType {
    /// `Target` is the **concrete** Python type we want to use
    /// to represent our Rust value.
    /// The underlying Rust type is a `u64`, so we'll convert it to a `PyInt`,
    /// a Python integer.
    type Target = PyInt;
    /// `Output`, instead, is a **wrapper** around the concrete type.
    /// It captures the ownership relationship between the Python object
    /// and the Python runtime.
    /// In this case, we're using a `Bound` smart pointer to a `PyInt`.
    /// The `'py` lifetime ensures that the Python object is owned 
    /// by the Python runtime.
    type Output = Bound<'py, PyInt>;
    /// Since the conversion can fail, we need to specify an error type.
    /// We can't fail to convert a `u64` into a Python integer,
    /// so we'll use `Infallible` as the error type.
    type Error = Infallible;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        // `u64` already implements `IntoPyObject`, so we delegate 
        // to its implementation to do the actual conversion.
        self.value.into_pyobject(py)
    }
}
```

### The `Output` associated type

Let's focus on the `Output` associated type for a moment.\
In almost all cases, you'll be setting `Output` to `Bound<'py, Self::Target>`[^syntax]. You're creating a new Python
object and its lifetime is tied to the Python runtime.

In a few cases, you might be able to rely on [`Borrowed<'a, 'py, Self::Target>`](https://docs.rs/pyo3/0.23.3/pyo3/prelude/struct.Borrowed.html)
instead.
It's slightly faster[^conversation], but it's limited to scenarios where you are borrowing from an existing Python object—fairly
rare for an `IntoPyObject` implementation.

There are no other options for `Output`, since `Output` must implement
[the `BoundObject` trait](https://docs.rs/pyo3/0.23.3/pyo3/trait.BoundObject.html),
the trait is [sealed](https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust/) and
those two types are the only implementors within `pyo3`.\
If it helps, think of `Output` as an enum with two variants: `Bound` and `Borrowed`.

## Provided implementations

`pyo3` provides out-of-the-box implementations of `IntoPyObject` for many Rust types, as well as for all `Py*` types.
Check out [its documentation](https://docs.rs/pyo3/0.23.3/pyo3/conversion/trait.IntoPyObject.html#foreign-impls)
for an exhaustive list.

[^syntax]: The actual syntax is a bit more complex: `type Output = Bound<'py, <Self as IntoPyObject<'py>>::Target>>;`.
We've simplified it for clarity.

[^conversation]: In addition to its documentation, you may find [this issue](https://github.com/PyO3/pyo3/issues/4467)
useful to understand the trade-offs between `&Bound` and `Borrowed`.
