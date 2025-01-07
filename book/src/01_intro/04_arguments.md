# Arguments

`no_op`, the function you added to solve the previous exercise, is _very_ simple:

```rust
use pyo3::prelude::*;
    
#[pyfunction]
fn no_op() {
    // Do nothing
}
```

Let's take it up a notch: what if you want to pass a value from Python to Rust?

## The `FromPyObject` trait

`#[pyfunction]` functions can take arguments, just like regular Rust functions.\
But there's a catch: it must be possible to build those arguments from Python objects.

The contract is encoded in the `FromPyObject` trait, defined in `pyo3`:

```rust
pub trait FromPyObject<'py>: Sized {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self>;
}
```

We won't go into the details of `FromPyObject`'s definition just yet: it would require an
in-depth discussion of Python's Global Interpreter Lock (GIL) and the way
`pyo3` models it in Rust. We'll get to it in the next section.\
For the time being, let's focus on what the trait unlocks for us: the ability to convert
Python objects into Rust types.

## Available implementations

`pyo3` provides implementations of `FromPyObject` for a large number of types—e.g. `i32`, `f64`, `String`, `Vec`, etc.
You can find an exhaustive list in [`pyo3`'s guide](https://pyo3.rs/v0.23.3/conversions/tables#argument-types),
under the "Rust" table column.

## Conversion cost

Going from a Python object to a Rust type is not free—e.g. the
in-memory representation of a Python list doesn't match the in-memory representation of a Rust `Vec`.\
The conversion introduces a (usually small) overhead that you'll have to incur every time you invoke
your Rust function from Python. It's a good trade-off if you end up performing enough
computational work in Rust to amortize the conversion cost.

## Python-native types

In `pyo3`'s documentation you can see a column of "Python-native" types.\
Don't try to use them to solve the exercise for this section: we'll cover them in the next one.

## References

- [The `FromPyObject` trait](https://docs.rs/pyo3/0.23.3/pyo3/conversion/trait.FromPyObject.html)
