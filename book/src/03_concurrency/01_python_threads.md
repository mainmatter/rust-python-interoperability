# Threads

## The overhead of multiprocessing

Let's have a look at the solution for the previous exercise:

```python
from multiprocessing import Process, Queue

def word_count(text: str, n_processes: int) -> int:
    result_queue = Queue()
    processes = []
    for chunk in split_into_chunks(text, n_processes):
        p = Process(target=word_count_task, args=(chunk, result_queue))
        p.start()
        processes.append(p)
    for p in processes:
        p.join()
    results = [result_queue.get() for _ in range(len(processes))]
    return sum(results)
```

Let's focus, in particular, on process creation:

```python
p = Process(target=word_count_task, args=(chunk, result_queue))
```

The parent process (the one executing `word_count`) doesn't share memory with the child process (the one
spawned via `p.start()`). As a result, the child process can't access `chunk` or `result_queue` directly.
Instead, it needs to be provided a **deep copy** of these objects[^pickle].\
That's not a major issue if the data is small, but it can become a problem on larger datasets.\
For example, if we're working with 8 GB of text, we'll end up with at least 16 GB of memory usage: 8 GB for the
parent process and 8 GB split among the child processes. Not ideal!

We could try to circumvent this issue[^mmap], but that's not always possible nor easy to do.\
A more straightforward solution is to use **threads** instead of processes.

## Threads

A **thread** is an execution context **within a process**.\
Threads share the same memory space and resources as the process that spawned them, thus allowing them to communicate
and share data with one another more easily than processes can.

```ascii
+------------------------+
|        Memory          |
|                        |
| +--------------------+ |
| |  Process A Space   | |  <-- Each process has its own memory space.
| |  +-------------+   | |      Threads share the same memory space
| |  | Thread 1    |   | |      of the process that spawned them.
| |  | Thread 2    |   | |
| |  | Thread 3    |   | |
| |  +-------------+   | |
| +--------------------+ |
|                        |
| +--------------------+ |
| |  Process B Space   | |
| |  +-------------+   | |
| |  | Thread 1    |   | |
| |  | Thread 2    |   | |
| |  +-------------+   | |
| +--------------------+ |
+------------------------+
```

Threads, just like processes, are operating system constructs.\
The operating system's scheduler is in charge of deciding which thread to run at any given time, partitioning CPU time
among them.

## The `threading` module

Python's `threading` module provides a high-level interface for working with threads.\
The API of the `Thread` class, in particular, mirrors what you already know from the `Process` class:

- A thread is created by calling the `Thread` constructor and passing it a target function to execute as well as
  any arguments that function might need.
- The thread is launched by calling its `start` method, and we can wait for it to finish by calling `join`.
- If we want to communicate between threads, we can use `Queue` objects, from the `queue` module, which are shared between threads.

## References:

- [`threading` module](https://docs.python.org/3/library/threading.html)
- [`Thread` class](https://docs.python.org/3/library/threading.html#threading.Thread)
- [`Queue` class](https://docs.python.org/3/library/queue.html)

[^pickle]: To be more precise, the `multiprocessing` module uses the `pickle` module to serialize the objects
that must be passed as arguments to the child process.
The serialized data is then sent to the child process, as a byte stream, over an operating system pipe.
On the other side of the pipe, the child process deserializes the byte stream back into Python objects using `pickle`
and passes them to the target function.\
This all system has higher overhead than a "simple" deep copy.

[^mmap]: Common workarounds include memory-mapped files and shared-memory objects, but these can be quite
difficult to work with. They also suffer from portability issues, as they rely on OS-specific features.
