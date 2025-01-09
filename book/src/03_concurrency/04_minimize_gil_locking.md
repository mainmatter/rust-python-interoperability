# Minimize GIL locking

All our examples so far fall into two categories:

- The Rust function holds the GIL for the entire duration of its execution.
- The Rust function doesn't hold the GIL at all, going straight into `Python::allow_threads` mode.

Real-world applications are often more nuanced, though.\
You'll need to hold the GIL for some operations (e.g. passing data back to Python), but you're able to release it
for others (e.g. long-running computations).

The goal is to minimize the time spent holding the GIL to the bare minimum, thus maximizing the potential
parallelism of your application.

## Strategy 1: isolate the GIL-free section

Let's look at an example: we're given a list of numbers and we need to modify it in place,
replacing each number with the result of an expensive computation that uses no Python objects.

To minimize GIL locking, we create Rust vector from the Python list, release the GIL, and perform the computation
and then re-acquire the GIL to update the Python list in place:

```rust
#[pyfunction]
fn update_in_place<'py>(
    python: Python<'py>,
    numbers: Bound<'py, PyList>
) -> PyResult<()> {
    // Holding the GIL
    let v: Vec<i32> = numbers.extract()?;
    let updated_v: Vec<_> = python.allow_threads(|| {
        v.iter().map(|&n| expensive_computation(n)).collect()
    });
    // Back to holding the GIL
    for (i, &n) in updated_v.iter().enumerate() {
        numbers.set_item(i, n)?;
    }
    Ok(()
}

fn expensive_computation(n: i32) -> i32 {
    // Some heavy number crunching
    // [...]
}
```

## Strategy 2: manually re-acquire the GIL inside the closure

In the example above, we've created a whole new vector to decouple the GIL-free section from the GIL-holding one.
If the input data is large, this can be a significant overhead.

Let's explore a different approach: we won't create a new pure-Rust vector.
Instead, we will re-acquire the GIL inside the closure—we'll hold it to access each list element and, after the computation is done,
update it in place. Nothing more.

Assuming you know nothing about `Ungil`, the naive solution might look like this:

```rust
#[pyfunction]
fn update_in_place<'py>(
    python: Python<'py>,
    numbers: Bound<'py, PyList>
) -> PyResult<()> {
    python.allow_threads(|| -> PyResult<()> {
        let n_numbers = numbers.len();
        for i in 0..n_numbers {
            let n = numbers.get_item(i)?.extract::<i64>()?;
            let result = expensive_computation(n);
            numbers.set_item(i, result))?;
        }
        Ok(())
    })
}
```

It won't compile, though. We're using a GIL-bound object (`numbers`) in a GIL-free section (inside `python.allow_threads`).
We need to **unbind** it first.

### `Py<T>` and `Bound<'py, T>`

Using `Bound<'py, T>::unbind` we get a `Py<T>` object back. It has no `'py` lifetime, it's no longer bound to the GIL.
We can try to use it in the GIL-free section:

```rust
#[pyfunction]
fn update_in_place<'py>(
    python: Python<'py>,
    numbers: Bound<'py, PyList>
) -> PyResult<()> {
    let numbers = numbers.unbind();
    python.allow_threads(|| -> PyResult<()> {
        let n_numbers = numbers.len();
        for i in 0..n_numbers {
            let n = numbers.get_item(i)?.extract::<i64>()?;
            let result = expensive_computation(n);
            numbers.set_item(i, result)?;
        }
        Ok(())
    })
}
```

But it won't compile either. `numbers.len()`, `numbers.get_item(i)`, and `numbers.set_item(i, result)` all require the GIL.
`Py<T>` is just a pointer to a Python object, it won't allow us to access it if we're not holding the GIL.

We need to **re-bind** it using a `Python<'py>` token, thus getting a `Bound<'py, PyList>` back.
How do we get a `Python<'py>` token inside the closure, though? Using `Python::with_gil`: it's the opposite
of `Python::allow_threads`, it makes sure to acquire the GIL before executing the closure and release it afterwards.
The closure is given a `Python` token as argument, which we can use to re-bind the `PyList` object:

```rust
#[pyfunction]
fn update_in_place<'py>(
    python: Python<'py>,
    numbers: Bound<'py, PyList>
) -> PyResult<()> {
    let n_numbers = numbers.len();
    let numbers_ref = numbers.unbind();
    // Release the GIL
    python.allow_threads(|| -> PyResult<()> {
        for i in 0..n_numbers {
            // Acquire the GIL again, to access the
            // i-th element of the list
            let n = Python::with_gil(|inner_py| {
                numbers_ref
                    .bind(inner_py)
                    .get_item(i)?
                    .extract::<i64>()
            })?;
            // Run the computation without holding the GIL
            let result = expensive_computation(n);
            // Re-acquire the GIL to update the list in place
            Python::with_gil(|inner_py| {
                numbers_ref.bind(inner_py).set_item(i, result)
            })?;
        }
        Ok(())
    })
}
```

## Be mindful of concurrency

The GIL is there for a reason: to protect Python objects from concurrent access.\
Whenever you release the GIL, you're allowing other threads to run and potentially modify the
Python objects you're working with.

In the examples above, another Python thread could modify the `numbers` list while we're computing the result.
E.g. it could remove an element, causing the index `i` to be out of bounds.

This is a common issue in multi-threaded programming, and it's up to you to handle it.\
Consider using synchronization primitives like [`Lock`](https://docs.python.org/3/library/threading.html#lock-objects)
to serialize access to the Python objects you're working with.
In other words, move towards fine-grained locking rather than the lock-the-world approach
you get with the GIL.

## References

- [`Py<T>` struct](https://docs.rs/pyo3/0.23.3/pyo3/struct.Py.html)
- [`Python::with_gil` method](https://docs.rs/pyo3/0.23.3/pyo3/marker/struct.Python.html#method.with_gil)
- [`Bound<'py, T>::unbind` method](https://docs.rs/pyo3/0.23.3/pyo3/prelude/struct.Bound.html#method.unbind)
