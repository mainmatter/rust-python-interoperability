# Concurrency

Up until now, we've kept things quite simple: all our code was designed for sequential execution, on both the Python and Rust side.\
It's time to spice things up a bit and explore concurrency!

In particular, we want to look at:

- How to run multithreaded routines in Rust, with Python code waiting for them to finish
- How to perform some processing in Rust, while allowing Python code to perform other tasks in the meantime
- How we can synchronize across threads in Rust, keeping Python's GIL in mind

We'll limit our exploration to threads, without venturing into the realm of `async`/`await`.

## Threads and processes

Throughout this chapter we'll often refer to **threads** and **processes**.\
Let's make sure we're all on the same page about what these terms mean before moving on.

### Processes

A **process** is an instance of a running program.\
The precise anatomy of a process depends on the underlying **operating system** (e.g. Windows or Linux).
Some characteristics are common across most operating systems, though. In particular, a process typically consists of:

- The program's code
- Its memory space, allocated by the operating system
- A set of resources (file handles, sockets, etc.)

There can be multiple processes running the same program, each with its own memory space and resources, fully
isolated from one another. 

### Threads

A **thread** is an execution context **within a process**.\
Threads share the same memory space and resources as the process that spawned them, thus allowing them to communicate 
and share data with one another more easily than processes can.

### Scheduling

Threads, just like processes, are a logical construct managed by the operating system.\
In the end, you can only run one set of instructions at a time on a CPU core, the physical execution unit.
Since there can be many more threads than there are CPU cores, the **operating system's scheduler** is in charge of 
deciding which thread to run at any given time, partitioning CPU time among them to maximize throughput and responsiveness.

## Python concurrency

Let's start by looking at Python's concurrency model.\
As we discussed in the [Global Interpreter Lock](../01_intro/05_gil.md) chapter,
Python's GIL prevents multiple threads from executing Python code simultaneously.

As a result, [thread-based parallelism](https://docs.python.org/3/library/threading.html) has historically
seen limited use in Python, as it doesn't provide the performance benefits one might expect from a
multithreaded application.

To work around the GIL, Python developers have turned to [**multiprocessing**](https://docs.python.org/3/library/multiprocessing.html):
rather than using multiple threads, they spawn multiple **processes**.
Each process has its own Python interpreter, and thus its own GIL. The operating system schedules these processes
independently, allowing them to run in parallel on multicore CPUs.

The multiprocessing paradigm is quite powerful, but it's not a good fit for every use case.
In particular, it's not well-suited for problems that require a lot of inter-process communication, since processes
don't share the same memory space. This can lead to performance bottlenecks and/or increased complexity[^mmap].  

That's where native extensions come in: they can **bypass the GIL** (under certain conditions) and allow us to run
multithreaded code, without the overhead of spawning and coordinating multiple processes.
We'll explore what this looks like for Rust in the next sections.

### Free-threading mode

Before moving on it's worth mentioning that Python's concurrency model is likely to undergo some significant changes 
in the future due to the introduction of [`CPython`'s free-threading mode](https://docs.python.org/3/howto/free-threading-python.html).
We won't cover it in this book, but it's worth keeping an eye on it as it matures out of the experimental phase.

[^mmap]: Common workaround include memory-mapped files and shared-memory objects, but these can be quite
  difficult to work with. They also suffer from portability issues, as they rely on OS-specific features.