# Modules

In Python, just like in Rust, your code is organized into modules.\
Your entire extension is a module!

That module is defined using `pyo3`'s `#[pymodule]` procedural macro, as
you've seen in the previous section:

```rust
#[pymodule]
fn setup(m: &Bound<'_, PyModule>) -> PyResult<()> {
   // [...]
}
```

`setup` becomes the entry point for the Python interpreter to load your extension.

## Naming matters

The name of the annotated function is important: there must be at least one module with a name that matches the name of
the dynamic library artifact that Python will try to load.
This is the name of the library target specified in your `Cargo.toml` file:

```toml
[lib]
name = "name_of_your_rust_library"
```

If you don't have a `[lib]` section, it defaults to the name of your package,
specified in the `[package]` section.

If the module name and the library name don't match, Python will raise an error when trying to import the module:

```text
ImportError: dynamic module does not define 
    module export function (PyInit_name_of_your_module)
```

## The `name` argument

You can also specify the name of the module explicitly using the `name` argument,
rather than relying on the name of the annotated function:

```rust
#[pymodule]
#[pyo3(name = "setup")]
fn random_name(m: &Bound<'_, PyModule>) -> PyResult<()> {
   // [...]
}
```

## Mysterious types?

You might be wondering: what's up with `&Bound<'_, PyModule>`? What about `PyResult`?\
Don't worry, we'll cover these types in due time later in the course.
Go with the flow for now!
