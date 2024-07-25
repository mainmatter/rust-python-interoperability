# Rust-Python interoperability

Welcome to **"Rust-Python interoperability"**!

In this course, you'll learn how to write Rust code that can be called from Python,
using `pyo3` to create Python extension modules.

We assume you are familiar with the basics of both Rust and Python, but we don't assume any prior interoperability knowledge.
We will provide explanations and references whenever we rely on advanced features in either language.

> [!NOTE]
> This course has been written by [Mainmatter](https://mainmatter.com/rust-consulting/).\
> It's one of the trainings in [our portfolio of Rust workshops](https://mainmatter.com/services/workshops/rust/).\
> Check out our [landing page](https://mainmatter.com/rust-consulting/) if you're looking for Rust consulting or
> training!

## Getting started

Run

```bash
# Install necessary tools
cargo install mdbook
cargo install --git https://github.com/mainmatter/100-exercises-to-learn-rust mdbook-exercise-linker

# Serve the book locally
cd book && mdbook serve --port 3012 --open
```

to open the companion book for this course in your browser.

## Requirements

- **Rust** (follow instructions [here](https://www.rust-lang.org/tools/install)).\
  If `rustup` is already installed on your system, run `rustup update` (or another appropriate command depending on how
  you installed Rust on your system)
  to make sure you're running on the latest stable version.
- _(Optional but recommended)_ An IDE with Rust autocompletion support.
  We recommend one of the following:
  - [RustRover](https://www.jetbrains.com/rust/);
  - [Visual Studio Code](https://code.visualstudio.com) with
    the [`rust-analyzer`](https://marketplace.visualstudio.com/items?itemName=matklad.rust-analyzer) extension.

## Solutions

You can find the solutions to the exercises in
the [`solutions` branch](https://github.com/mainmatter/rust-python-interoperability/tree/solutions) of this repository.

# License

Copyright © 2024- Mainmatter GmbH (https://mainmatter.com), released under the
[Creative Commons Attribution-NonCommercial 4.0 International license](https://creativecommons.org/licenses/by-nc/4.0/).
