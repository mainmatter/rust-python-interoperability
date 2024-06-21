# Anatomy of a Python extension

Before we dive into the details of writing Python extensions with Rust,
let's take a look at the **project structure**.

## `maturin`

We'll use `maturin` to build, package and publish Python extensions written in Rust.
Let's install it:

```bash
rye install "maturin>=1.6"
```

`maturin` can be used to scaffold a brand-new extension:

```bash
# You can use a different binding library rather than `pyo3`, 
# but we won't cover those alternatives in this course.
maturin new --bindings="pyo3" my_extension
```

`maturin new` is what we used to generate the exercise for this section.\
In the scaffolded project you'll find a Rust package alongside a `pyproject.toml` file.

Let's start by exploring the Rust package.

## `Cargo.toml`

The manifest file, `Cargo.toml`, looks like this:

```toml
[package]
name = "setup"
version = "0.1.0"
edition = "2021"

[lib]
name = "setup"
crate-type = ["cdylib"]

[dependencies]
pyo3 = "0.21.1"
```

Two things stand out in this file, compared to a regular Rust project:

- The `crate-type` attribute is set to `cdylib`.
- The `pyo3` crate is included as a dependency.

Let's cover these two points in more detail.

### `cdylib`

By default, Rust libraries are compiled as **static libraries**.\
To create a Python extension, we need to compile the Rust code as a **dynamic library**.

This is done by setting the `crate-type` attribute to `cdylib` in the `Cargo.toml` file:

```toml
[lib]
crate-type = ["cdylib"]
```

If you want to use the same crate as both a Rust library and a Python extension, you can use the `rlib` crate type for the Rust library and `cdylib` for the Python extension.

```toml
[lib]
crate-type = ["rlib", "cdylib"]
```

## References

- [Linking artifacts together (Rust compiler documentation)](https://doc.rust-lang.org/reference/linkage.html)
