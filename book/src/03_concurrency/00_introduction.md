# Concurrency

All our code so far has been designed for sequential execution, on both the Python and Rust side.
It's time to spice things up a bit and explore concurrency[^scope]!

We won't dive straight into Rust this time.\
We'll start by solving a few parallel processing problems in Python, to get a feel for Python's capabilities and limitations.
Once we have a good grasp of what's possible there, we'll port our solutions over to Rust.

## Multiprocessing

If you've ever tried to write parallel code in Python, you've probably come across the `multiprocessing` module.
Before we dive into the details, let's take a step back and review the terminology we'll be using.

### Processes

A **process** is an instance of a running program.\
The precise anatomy of a process depends on the underlying **operating system** (e.g. Windows or Linux).
Some characteristics are common across most operating systems, though. In particular, a process typically consists of:

- The program's code
- Its memory space, allocated by the operating system
- A set of resources (file handles, sockets, etc.)

```ascii
+------------------------+
|        Memory          |
|                        |
| +--------------------+ |
| |  Process A Space   | |  <-- Each process has a separate memory space.
| +--------------------+ |
|                        |
| +--------------------+ |
| |  Process B Space   | |
| |                    | |
| +--------------------+ |
|                        |
| +--------------------+ |
| |  Process C Space   | |
| +--------------------+ |
+------------------------+
```

There can be multiple processes running the same program, each with its own memory space and resources, fully
isolated from one another.\
The **operating system's scheduler** is in charge of deciding which process to run at any given time, partitioning CPU time
among them to maximize throughput and/or responsiveness.

### The `multiprocessing` module

Python's `multiprocessing` module allows us to spawn new processes, each running its own Python interpreter.

A process is created by invoking the `Process` constructor with a target function to execute as well as
any arguments that function might need.
The process is launched by calling its `start` method, and we can wait for it to finish by calling `join`.

If we want to communicate between processes, we can use `Queue` objects, which are shared between processes.
These queues try to abstract away the complexities of inter-process communication, allowing us to pass messages
between our processes in a relatively straightforward manner.

## References:

- [`multiprocessing` module](https://docs.python.org/3/library/multiprocessing.html)
- [`Process` class](https://docs.python.org/3/library/multiprocessing.html#multiprocessing.Process)
- [`Queue` class](https://docs.python.org/3/library/multiprocessing.html#multiprocessing.Queue)

[^scope]: We'll limit our exploration to threads and processes, without venturing into the realm of `async`/`await`.
