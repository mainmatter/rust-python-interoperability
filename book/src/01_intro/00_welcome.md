# Welcome

Welcome to **"Rust-Python interoperability"**!\
This course will teach you how to call Rust code from Python, packaged as a native extension module.

We assume you are familiar with both Rust and Python, but we don't assume any prior interoperability knowledge. We will provide a brief explanation and references whenever we rely on advanced language features in either language.

## Methodology

This course is based on the "learn by doing" principle.\
You'll build up your knowledge in small, manageable steps. It has been designed to be interactive and hands-on.

[Mainmatter](https://mainmatter.com/rust-consulting/) developed this course
to be delivered in a classroom setting, over a whole day: each attendee advances
through the lessons at their own pace, with an experienced instructor providing
guidance, answering questions and diving deeper into the topics as needed.\
If you're interested in attending one of our training sessions, or if you'd like to
bring this course to your company, please [get in touch](https://mainmatter.com/contact/).

You can also follow the course on your own, but we recommend you find a friend or
a mentor to help you along the way should you get stuck. You can
also find solutions to all exercises in the
[`solutions` branch of the GitHub repository](https://github.com/mainmatter/rust-python-interoperability/tree/solutions).

## Prerequisites

To follow this course, you must install:

- [Rust](https://www.rust-lang.org/tools/install)
- [`uv`](https://docs.astral.sh/uv/), a Python package manager

If Rust is already installed on your machine, make sure to update it to the latest version:

```bash
# If you installed Rust using `rustup`, the recommended way,
# you can update to the latest stable toolchain with:
rustup update stable
```

These commands should successfully run on your machine:

```bash
cargo --version
uv --version
```

Don't start the course until you have these tools installed and working.

## Structure

On the left side of the screen, you can see that the course is divided into sections.\
To verify your understanding, each section is paired with an exercise that you need to solve.

You can find the exercises in the
[companion GitHub repository](https://github.com/mainmatter/rust-python-interoperability).\
Before starting the course, make sure to clone the repository to your local machine:

```bash
# If you have an SSH key set up with GitHub
git clone git@github.com:mainmatter/rust-python-interoperability.git
# Otherwise, use the HTTPS URL:
#
#   git clone https://github.com/mainmatter/rust-python-interoperability.git
```

We recommend you work on a branch, so you can easily track your progress and pull
updates from the main repository if needed:

```bash
cd rust-python-interoperability
git checkout -b my-solutions
```

All exercises are located in the `exercises` folder.
Each exercise is structured as a Rust package.
The package contains the exercise itself, instructions on what to do (in `src/lib.rs`), and a test suite to
automatically verify your solution.

### `wr`, the workshop runner

To verify your solutions, we've also provided a tool to guide you through the course: the `wr` CLI, short for "workshop runner".
Install `wr` by following the instructions on [its website](https://mainmatter.github.io/rust-workshop-runner/).

Once you have `wr` installed, open a new terminal and navigate to the top-level folder of the repository.
Run the `wr` command to start the course:

```bash
wr
```

`wr` will verify the solution to the current exercise.\
Don't move on to the next section until you've solved the exercise for the current one.

> We recommend committing your solutions to Git as you progress through the course,
> so you can easily track your progress and "restart" from a known point if needed.

Enjoy the course!

## Author

This course was written by [Luca Palmieri](https://www.lpalmieri.com/), Principal Engineering
Consultant at [Mainmatter](https://mainmatter.com/rust-consulting/).\
Luca has been working with Rust since 2018, initially at TrueLayer and then at AWS.\
Luca is the author of ["Zero to Production in Rust"](https://zero2prod.com),
the go-to resource for learning how to build backend applications in Rust, and ["100 Exercises to Learn Rust"](https://rust-exercises.com), a learn-by-doing introduction to Rust itself.\
He is also the author and maintainer of a variety of open-source Rust projects, including
[`cargo-chef`](https://github.com/LukeMathWalker/cargo-chef),
[Pavex](https://pavex.dev) and [`wiremock`](https://github.com/LukeMathWalker/wiremock-rs).
