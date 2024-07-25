from multiprocessing import Process, Queue


# Return the number of words in `text` using `n_processes` processes.
# You'll need to:
# - create a result queue to store the results of each process
# - launch up to `n` processes in a loop, storing each process handle in a list
# - join each process in a loop, to wait for them to finish
# - drain the result queue into a list
# - sum the results in the list to get the final count
#
# We provide a function to split the text into chunks as well as
# a function to perform the counting in each process.
#
# Relevant links:
# - https://docs.python.org/3/library/multiprocessing.html
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


# Compute the number of words in `text` and push the result into `result_queue`.
# This function should be used as the target function for a `Process`.
def word_count_task(text: str, result_queue: 'Queue[int]') -> None:
    n_words = len(text.split())
    result_queue.put(n_words)


# Splits a string into `n` chunks, ensuring splits occur at whitespace.
def split_into_chunks(s: str, n: int):
    if n <= 0:
        raise ValueError("Number of chunks 'n' must be greater than 0")

    avg_length = len(s) // n
    length = len(s)
    start = 0

    for _ in range(n):
        if start >= length:
            return  # No more content to yield

        # Calculate tentative end index
        end = start + avg_length

        # Ensure we don't exceed the string length
        if end >= length:
            yield s[start:]
            return

        # Adjust the end index to the nearest whitespace
        while end < length and not s[end].isspace():
            end += 1

        # If no whitespace was found, return the rest of the string
        if end == length:
            yield s[start:]
            return

        # Yield the chunk and update the start index
        yield s[start:end].strip()
        start = end + 1  # Move past the whitespace
