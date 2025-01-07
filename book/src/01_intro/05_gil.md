# Global Interpreter Lock (GIL)

If you go back to [`pyo3`'s documentation on arguments](https://pyo3.rs/v0.23.3/conversions/tables#argument-types),
you'll find a table column listing so called "Python-native" types.
What are they, and why would you use them?

## Python-native types

There is overhead in converting a Python object into a Rust-native type.\
That overhead might dominate the cost of invoking your Rust function if the function itself isn't doing much
computational work. In those cases, it can be desirable to work directly using Python's in-memory representation of the object.
That's where the `Py*` types come in: they give you direct access to Python objects, with minimal overhead[^type-check].

Out of the entire family of `Py*` types, `PyAny` deserves a special mention.
It's the most general Python-native type in `pyo3`: it stands for an arbitrary Python object.
You can use it whenever you don't know the exact type of the object you're working with, or you don't care about it.

## `Py*` types don't implement `FromPyObject`

Let's try to rewrite the solution of the previous exercise using `PyList` rather than `Vec<u64>`:

```rust
use pyo3::prelude::*;

fn print_number_list(list: &PyList) {
    todo!()
}
```

If you try to compile this code, you'll get an error:

```text
error[E0277]: the trait bound `&PyList: PyFunctionArgument<'_, '_>` is not satisfied
   --> src/lib.rs:7:28
    |
7   | fn print_number_list(list: &PyList) {
    |                            ^ 
    |        the trait `PyClass` is not implemented for `&PyList`, 
    |        which is required by `&PyList: PyFunctionArgument<'_, '_>`
    |
    = help: the following other types implement trait `PyFunctionArgument<'a, 'py>`:
              &'a pyo3::Bound<'py, T>
              Option<&'a pyo3::Bound<'py, T>>
    = note: required for `&PyList` to implement `FromPyObject<'_>`
    = note: required for `&PyList` to implement `FromPyObjectBound<'_, '_>`
    = note: required for `&PyList` to implement `PyFunctionArgument<'_, '_>`
```

The error message is a bit cryptic because it mentions a number of private `pyo3` traits (`PyFunctionArgument` and
`FromPyObjectBound`), but the gist of it is that `&PyList` doesn't implement `FromPyObject`. That's
true for all `Py*` types.

Confusing, isn't it? How is possible that Python-native types, that require **no conversion**, don't implement the
trait that allows you to convert Python objects into Rust types?

It's time to have _that_ talk, the one about Python's Global Interpreter Lock (GIL).

## Global Interpreter Lock (GIL)

Out of the box, Python's[^cpython] data structures are not thread-safe. To prevent data races, there is a global
mutual exclusion lock that allows only one thread to execute Python bytecode at a time—i.e. the so-called
Global Interpreter Lock (GIL).

**It is forbidden to interact with Python objects without holding the GIL.**

That's why `pyo3` doesn't implement `FromPyObject` for `Py*` types: it would allow you to interact with Python objects
without you necessarily holding the GIL, a recipe for disaster.

## `Python<'py>`

`pyo3` uses a combination of lifetimes and smart pointers to ensure that you're interacting with Python objects
in a safe way.

`Python<'py>` is the cornerstone of the entire system: it's a **token type** that guarantees that you're holding the GIL.
All APIs that require you to hold the GIL will, either directly or indirectly, require you to provide a `Python<'py>`
token as proof.

`pyo3` will automatically acquire the GIL behind the scenes whenever you invoke a Rust function from Python. In fact,
you can ask for a `Python<'py>` token as an argument to your Rust function, and `pyo3` will provide it for you—it has
no (additional) cost.

```rust
use pyo3::prelude::*;
// There is no runtime difference between invoking the two functions
// below from Python.
// The first one is just more explicit about the fact that it requires
// the caller to acquire the GIL ahead of time.

#[pyfunction]
fn print_number_list(_py: Python<'_>, list: Vec<u64>) {
    todo!()
}

#[pyfunction]
fn print_number_list2(list: Vec<u64>) {
    todo!()
}
```

`'py`, the lifetime parameter of `Python<'py>`, is used to represent how long the GIL is going to be held.

### `Bound<'py>`

You won't be interacting with `Python<'py>` directly most of the time.\
Instead, you'll use the `Bound<'py, T>` type, a smart pointer that encapsulates a reference to a Python object, ensuring
that you're holding the GIL when you're interacting with it.

Using `Bound<'py, T>` we can finally start using the `Py*` types as function arguments:

```rust
use pyo3::prelude::*;

#[pyfunction]
fn print_number_list(list: Bound<'_, PyList>) {
    todo!()
}
```

`Bound` ensures that we're holding the GIL when interacting with the list instance that has been passed to us
as function argument.

### `FromPyObject`

We can now go back to the definition of the `FromPyObject` trait:

```rust
pub trait FromPyObject<'py>: Sized {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self>;
}
```

`extract_bound` takes a `&Bound<'py, PyAny>` as argument, rather than a bare `&PyAny`, to ensure that we're holding the GIL
when we're interacting with the Python object during the conversion.

## References

- [`FromPyObject`](https://docs.rs/pyo3/0.23.3/pyo3/conversion/trait.FromPyObject.html)
- [`Python<'py>`](https://docs.rs/pyo3/0.23.3/pyo3/marker/struct.Python.html)
- [Global Interpreter Lock](https://docs.python.org/3/c-api/init.html#thread-state-and-the-global-interpreter-lock)
- [Official guidance on Python-native vs Rust-native types](https://pyo3.rs/v0.23.3/conversions/tables#using-rust-library-types-vs-python-native-types)

## Footnotes

[^type-check]: `pyo3` still needs to ensure that the Python object you're working with is of the expected type.
It'll therefore perform an `isinstance` check before handing you the object—e.g.
checking that an object is indeed a list before giving you a `PyList` argument. The only exception to this rule
is `PyAny`, which can represent an arbitrary Python object.

[^cpython]: CPython is the reference implementation of Python, written in C.
It's the most widely used Python interpreter and what most people refer to when they say "Python".
