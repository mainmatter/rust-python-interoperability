# The GIL problem

## Concurrent, yes, but not parallel

On the surface, our thread-based solution addresses all the issues we identified in the `multiprocessing` module:

```python
from threading import Process
from queue import Queue

def word_count(text: str, n_threads: int) -> int:
    result_queue = Queue()
    threads = []

    for chunk in split_into_chunks(text, n_threads):
        t = Thread(target=word_count_task, args=(chunk, result_queue))
        t.start()
        threads.append(t)

    for t in threads:
        t.join()

    results = [result_queue.get() for _ in range(len(threads))]
    return sum(results)
```

When a thread is created, we are no longer cloning the text chunk nor incurring the overhead of inter-process communication:

```python
t = Thread(target=word_count_task, args=(chunk, result_queue))
```

Since the spawned threads share the same memory space as the parent thread, they can access the `chunk` and `result_queue` directly.

Nonetheless, there's a major issue with this code: **it won't actually use multiple CPU cores**.\
It will run sequentially, even if we pass `n_threads > 1` and multiple CPU cores are available.

## Python concurrency

You guessed it: the infamous Global Interpreter Lock (GIL) is to blame.
As we discussed in the [GIL chapter](../01_intro/05_gil.md),
Python's GIL prevents multiple threads from executing Python code simultaneously[^free-threading].

As a result, thread-based parallelism has historically
seen limited use in Python, as it doesn't provide the performance benefits one might expect from a
multithreaded application.

That's why the `multiprocessing` module is so popular: it allows Python developers to bypass the GIL.
Each process has its own Python interpreter, and thus its own GIL. The operating system schedules these processes
independently, allowing them to run in parallel on multicore CPUs.

But, as we've seen, multiprocessing comes with its own set of challenges.

## Native extensions

There's a third way to achieve parallelism in Python: **native extensions**.\
We must [be holding the GIL](../01_intro/05_gil.html#pythonpy) when we invoke a Rust function from Python, but
pure Rust threads are not affected by the GIL, as long as they don't need to interact with Python objects.

Let's rewrite again our `word_count` function, this time in Rust!

[^free-threading]: This is the current state of Python's concurrency model. There are some exciting changes on the horizon, though!
[`CPython`'s free-threading mode](https://docs.python.org/3/howto/free-threading-python.html) is an experimental feature
that aims to remove the GIL entirely.
It would allow multiple threads to execute Python code simultaneously, without forcing developers to rely on multiprocessing.
We won't cover the new free-threading mode in this course, but it's worth keeping an eye on it as it matures out of the experimental phase.
