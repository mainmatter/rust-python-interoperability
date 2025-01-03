# Anatomy of a Python extension

<div class="warning">

Don't jump ahead!\
Complete the exercise for the previous section before you start this one.\
It's located in `exercises/01_intro/00_welcome`, in the [course GitHub's repository](https://github.com/mainmatter/rust-python-interoperability).\
Use [`wr`](00_welcome.md#wr-the-workshop-runner) to start the course and verify your solutions.

</div>

To invoke Rust code from Python we need to create a **Python extension module**.

> Rust, just like C and C++, compiles to native code. For this reason, extension
> modules written in Rust are often called **native extensions**. Throughout this course
> we'll use the terms **Python extension**, **Python extension module** and **native extension** interchangeably.

## `maturin`

We'll use `maturin` to build, package and publish Python extensions written in Rust. Let's install it:

```bash
uv tool install "maturin>=1.8"
```

Tools installed via `uv` should be available in your path. Run:

```bash
uv tool update-shell
```

to make sure that's the case.

## Exercise structure

All exercises in this course will follow the same structure:

- an extension module written in Rust, in the root of the exercise directory
- a Python package that invokes the functionality provided by the extension, in the `sample` subdirectory

The extension module will usually be tested from Python, in the `sample/tests` subdirectory.
You will have to modify the Rust code in the extension module to make the tests pass.

## Extension structure

Let's explore the structure of the extension module for this section.

```plaintext
01_setup
├── sample
├── src
│   └── lib.rs
├── Cargo.toml
└── pyproject.toml
```

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
pyo3 = "0.23.0"
```

Two things stand out in this file compared to a regular Rust project:

- The `crate-type` attribute is set to `cdylib`.
- The `pyo3` crate is included as a dependency.

Let's cover these two points in more detail.

## Linking

### Static linking

By default, Rust libraries are compiled as **static libraries**.\
All dependencies are linked into the final executable at compile-time, making the executable self-contained[^static].

That's great for distributing applications, but it's not ideal for Python extensions.\
To perform static linking, the extension module would have to be compiled alongside the Python interpreter.
Furthermore, you'd have to distribute the modified interpreter to all your users.\
At the ecosystem level, this process would scale poorly, as each user needs to leverage
several unrelated extensions at once. Every single project would have to compile its own
bespoke Python interpreter.

### Dynamic linking

To avoid this scenario, Python extensions are packaged as **dynamic libraries**.\
The Python interpreter can load these libraries at runtime, without having to be recompiled.
Instead of distributing a modified Python interpreter to all users, you must now distribute
the extension module as a standalone file.

Rust supports dynamic linking, and it provides two different flavors of dynamic libraries: `dylib` and `cdylib`.
`dylib` are Rust-flavored dynamic libraries, geared towards Rust-to-Rust dynamic linking.
`cdylib`, on the other hand, are dynamic libraries that export a C-compatible interface (**C** **dy**namic **lib**rary).

You need a common dialect to get two different languages to communicate with each other. They
both need to speak it and understand it.\
That bridge, today, is C's ABI (**A**pplication **B**inary **I**nterface).

That's why, for Python extensions, you must use the `cdylib` crate type:

```toml
[lib]
crate-type = ["cdylib"]
```

## `pyo3`

It's not enough to expose a C-compatible interface.
You must also comply with the [Python C API](https://docs.python.org/3/c-api/index.html), the interface Python uses to interact with C extensions.

Doing this manually is error-prone and tedious.
That's where the `pyo3` crate comes in: it provides a safe and idiomatic way to write Python extensions in Rust, abstracting away the low-level details.

In `lib.rs`, you can see it in action:

```rust
use pyo3::prelude::*;

#[pyfunction]
fn it_works() -> bool {
    todo!()
}

/// A Python module implemented in Rust.
#[pymodule]
fn setup(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(it_works, m)?)?;
    Ok(())
}
```

We're using `pyo3` to define a Python function, named `it_works`, that returns a boolean.
The function is then exposed to Python at the top-level of our extension module, named `setup`.

That same function is then invoked from Python, inside `sample/tests/test_sample.py`:

```python
from setup import it_works

def test_works(): 
    assert it_works()
```

We'll cover the details of `#[pyfunction]` and `#[pymodule]` in the next section, no worries.

## `pyproject.toml`

Before we move on, let's take a look at `pyproject.toml`, the Python "manifest" of the extension module:

```toml
[build-system]
requires = ["maturin>=1.8,<2.0"]
build-backend = "maturin"

[project]
name = "setup"
# [...]
requires-python = ">=3.13"

[tool.maturin]
features = ["pyo3/extension-module"]
```

It specifies the build system, the extension name and version, the required Python version, and the features to enable when building the extension module.
This is what `uv` looks at when building the extension module, before delegating the build
process to `maturin`, which in turn invokes `cargo` to compile the Rust code.

## What do I need to do?

A lot has to go right behind the scenes to make a Python extension work.\
That's why the exercise for this section is fairly boring—we want to verify
that you can build and test a Python extension module without issues.

Things will get a lot more interesting over the coming sections, I promise!

## References

- [Linking artifacts together (Rust compiler documentation)](https://doc.rust-lang.org/reference/linkage.html)
- [Python C API](https://docs.python.org/3/c-api/index.html)

## Footnotes

[^static]: This is true up to an extent. In most cases, some dependencies are still dynamically linked, e.g. [libc](https://en.wikipedia.org/wiki/C_standard_library) on most Unix systems. Nonetheless, the final executable is self-contained in the sense that it doesn't rely on the presence of the Rust standard library or any other Rust crate on the user's
system.
