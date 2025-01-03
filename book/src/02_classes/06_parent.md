# Parent class

Let's go back to our example from the previous section:

```rust
use pyo3::prelude::*;

#[pyclass(subclass)]
struct Parent {
    name: String,
}

#[pymethods]
impl Parent {
    #[new]
    fn new(name: String) -> Self {
        // [...]
    }

    fn greet(&self) {
        println!("Hello, {}!", self.name);
    }
}

#[pyclass(extends=Parent)]
struct Child {
    age: u8,
}

#[pymethods]
impl Child {
    #[new]
    fn new(name: String, age: u8) -> PyClassInitializer<Self> {
        // [...]
    }
}
```

`Child.greet` is not defined, therefore it falls back to the `Parent.greet` method.\
What if we wanted to override it in `Child`?

## Overriding methods

On the surface, it's simple: just define a method with the same name in the subclass.

```rust
#[pymethods]
impl Child {
    #[new]
    fn new(name: String, age: u8) -> PyClassInitializer<Self> {
        // [...]
    }
    
    fn greet(&self) {
        println!("Hi, I'm {} and I'm {} years old!", self.name, self.age);
    }
}
```

There's an issue though: `self.name` won't work because the Rust struct for `Child` doesn't have a `name` field.
At the same time, the Python `Child` class does, because it inherits it from `Parent`.

How do we fix this?

## `as_super` to the rescue

We need a way, in Rust, to access the fields and methods of the parent class from the child class.\
This can be done using another one of `pyo3`'s smart pointers: `PyRef`.

```rust
#[pymethods]
impl Child {
    // [...]
    
    fn greet(self_: PyRef<'_, Self>) {
        todo!()
    }
}
```

`PyRef` represents an immutable reference to the Python object.\
It allows us, in particular, to call the [`as_super`](https://docs.rs/pyo3/0.23.3/pyo3/pycell/struct.PyRef.html#method.as_super) method,
which returns a reference to the parent class.

```rust
#[pymethods]
impl Child {
    // [...]
    
    fn greet(self_: PyRef<'_, Self>) {
        // This is now a reference to a `Parent` instance!
        let parent = self_.as_super();
        println!("Hi, I'm {} and I'm {} years old!", parent.name, self_.age);
    }
}
```

Now we can access the `name` field from the parent class, and the `age` field from the child class.

## `PyRef` and `PyRefMut`

`PyRef` is for immutable references, but what if we need to modify the parent class?\
In that case, we can use `PyRefMut`, which is a mutable reference.
