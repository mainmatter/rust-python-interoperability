# Functions

Empty modules are not that useful: let's add some functions to our extension!\
As you've seen in the ["Setup" section](01_setup.md), `pyo3` provides another procedural macro
to define functions that can be called from Python: `#[pyfunction]`.

Back then we used it to define the `it_works` function:

```rust
use pyo3::prelude::*;

// 👇 A Python function defined in Rust
#[pyfunction]
fn it_works() -> bool {
    true
}
```

Unlike modules, functions aren't exposed to Python automatically; you must
attach them to a module using the `wrap_pyfunction!` macro:

```rust
#[pymodule]
fn setup(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // 👇 Expose the function to Python
    m.add_function(wrap_pyfunction!(it_works, m)?)?;
    Ok(())
}
```
