# Immutable types

Concurrency introduces many new classes of bugs that are not present in single-threaded programs.
Data races are one of the most common: two threads try to access the same memory location at the same time, and at least one of them
is writing to it. What should happen?\
In most programming languages, the behavior is undefined: the program could crash, or it could produce incorrect results.

Data races can't happen in a single-threaded program, because only one thread can access the memory at a time.
That's where the GIL comes in: since it serializes the execution of code that accesses Python objects,
it prevents all kinds of data races (albeit with a significant performance cost).

There's another way to prevent data races though: by making sure that the data is immutable.
There's no need for synchronization if the data can't change!

## Built-in immutable types

Python has many immutable types—e.g. `int`, `float`, `str`.\
Whenever you modify them, you're actually creating a new object, not changing the existing one.

```python
a = 1
b = a
a += 1

assert a == 2
# a is a new object,
# b is still 1
assert b == 1
```

Since they're immutable, they're considered **thread-safe**: you can access them from multiple threads
without worrying about data races and synchronization.

## Frozen dataclasses

You can define your own immutable types in Python using `dataclasses` and the `frozen` attribute.

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class Point:
    x: int
    y: int

p = Point(1, 2)
# This will raise a `FrozenInstanceError` exception
p.x = 3
```

The `frozen` attribute makes the class immutable: you can't modify its attributes after creation.
This goes beyond modifying the _values_ of the existing attributes. You are also forbidden from
adding new attributes, e.g.:

```python
# This will raise a `FrozenInstanceError` exception
# But would work if `frozen=False` or for a "normal"
# class without the `@dataclass` decorator
p.z = 3
```

### In Rust

Let's see how we can define a similar immutable type in Rust.

```rust
use pyo3::prelude::*;

#[pyclass(frozen)]
struct Point {
    x: i32,
    y: i32,
}
```

The above is not enough to get all the niceties of Python's `dataclasses`, but
it's sufficient to make the class immutable.\
If a `pyclass` is marked as `frozen`, `pyo3` will allow us to access its fields without
holding the GIL—i.e. via `Py<T>` instead of `Bound<'py, T>`

```rust
#[pyfunction]
fn print_point<'py>(python: Python<'py>, point: Bound<'py, Point>) {
    let point: Py<Point> = point.unbind();
    python.allow_threads(|| {
        // We can now access the fields of the Point struct
        // even though we are not holding the GIL
        let point: &Point = point.get();
        println!("({}, {})", point.x, point.y);
    });
}
```

This wouldn't compile if `Point` wasn't marked as `frozen`, thanks to `Py<T>::get`'s signature:

```rust
impl<T> Py<T>
where
    T: PyClass,
{
    pub fn get(&self) -> &T
    where
        // `Frozen = True` is where the magic happens!
        T: PyClass<Frozen = True> + Sync,
    { /* ... */ }
}
```

## Summary

Immutable types significantly simplify GIL jugglery in `pyo3`. If it fits the constraints of the problem you're solving,
consider using them to make your code easier to reason about (and potentially faster!).
