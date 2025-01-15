# Modify the Rust extension to get the test below to pass
# Do NOT modify the test itself!
import pytest
import timeit
import math

from release import nth_prime
from concurrent.futures.thread import ThreadPoolExecutor
from concurrent.futures import wait

def parallel(executor: 'ThreadPoolExecutor', n: int):
    future1 = executor.submit(nth_prime, n)
    future2 = executor.submit(nth_prime, n)
    wait([future1, future2], return_when='ALL_COMPLETED')

def serial(executor: 'ThreadPoolExecutor', n: int):
    future = executor.submit(nth_prime, n)
    future.result()


def test_timing():
    # Record how long it takes to compute the n-th prime for a sufficiently
    # high `n`.
    # Then time how long it takes to run two of those computations in parallel
    # with the same input. Ensure that the parallel version doesn't take 2x as long
    n = 1600
    n_executions = 10000

    executor = ThreadPoolExecutor(max_workers=2)

    serial_timing = timeit.timeit(lambda: serial(executor, n), number=n_executions) / n_executions
    parallel_timing = timeit.timeit(lambda: parallel(executor, n), number=n_executions) / n_executions
    print(f"Serial timing: {serial_timing}")
    print(f"Parallel timing: {parallel_timing}")
    assert math.isclose(parallel_timing, serial_timing, rel_tol=0.10)
