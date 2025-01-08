# Python's threads

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

We could try to circumvent this issue by using [shared memory](https://docs.python.org/3/library/multiprocessing.shared_memory.html),
but that's not always possible nor easy to do.

## Threads to the rescue

As we discussed in the previous chapter, distinct processes don't share memory. Threads within the same process, on the other hand, do.
If we restructure our solution to use threads instead of processes, we can avoid the overhead of deep copying data.

Let's try!


[^pickle]: To be more precise, the `multiprocessing` module uses the `pickle` module to serialize the objects
   that must be passed as arguments to the child process.
   The serialized data is then sent to the child process, as a byte stream, over an operating system pipe.
   On the other side of the pipe, the child process deserializes the byte stream back into Python objects using `pickle`
   and passes them to the target function.\
   This all system has higher overhead than a "simple" deep copy.
