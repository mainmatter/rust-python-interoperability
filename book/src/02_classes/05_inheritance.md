# Inheritance

Python, unlike Rust, supports inheritance.\
Each class in Python can inherit attributes and methods from a **parent class**.

```python
class Parent:
    def __init__(self, name):
        self.name = name

    def greet(self):
        print(f"Hello, {self.name}!")
        
# Declare `Child` as a subclass of `Parent`
class Child(Parent):
    def __init__(self, name, age):
        # Call the parent class's constructor
        super().__init__(name)
        self.age = age
        
child = Child("Alice", 7)
# `Child` inherits the `greet` method from `Parent`, so we can call it
child.greet() # Prints "Hello, Alice!"
```

## `pyo3` and inheritance

`pyo3` supports inheritance as well, via additional attributes on the `#[pyclass]` macro.\
To understand how it works, let's try to translate the Python example above to Rust. We'll start with defining
the base class, `Parent`:

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
        Parent { name }
    }

    fn greet(&self) {
        println!("Hello, {}!", self.name);
    }
}
```

You can spot one new attribute in the `#[pyclass]` macro: `subclass`. This attribute tells `pyo3` that this class
can be subclassed, and it should generate the necessary machinery to support inheritance.

Now let's define the `Child` class, which inherits from `Parent`:

```rust
#[pyclass(extends=Parent)]
struct Child {
    age: u8,
}
```

We're using the `extends` attribute to specify that `Child` is a subclass of `Parent`.\
Things get a bit more complicated when it comes to the constructor:

```rust
#[pymethods]
impl Child {
    #[new]
    fn new(name: String, age: u8) -> PyClassInitializer<Self> {
        let parent = Parent::new(name);
        let child = Self { age };
        PyClassInitializer::from(parent).add_subclass(child)
    }
}
```

Whenever you initialize a subclass, you need to make sure that the parent class is initialized first.\
We start by calling `Parent::new` to create an instance of the parent class. We then initialize `Child`, via `Self { age }`.
We then use `PyClassInitializer` to return both the parent and child instances together.

Even though `Child` doesn't have a `greet` method on the Rust side, you'll be able to call it from Python since the
generated `Child` class inherits it from `Parent`.

## Nested inheritance

`PyClassInitializer` can be used to build arbitrarily deep inheritance hierarchies.
For example, if `Child` had its own subclass, you could call `add_subclass` again to add yet another subclass to the chain.

```rust
#[pyclass(extends=Child)]
struct Grandchild {
    hobby: String,
}

#[pymethods]
impl Grandchild {
    #[new]
    fn new(name: String, age: u8, hobby: String) -> PyClassInitializer<Self> {
        let child = Child::new(name, age);
        let grandchild = Self { hobby };
        PyClassInitializer::from(child).add_subclass(grandchild)
    }
}
```

### Limitations

`pyo3` supports two kinds of superclasses:

- A Python class defined in Rust, via `#[pyclass]`
- A Python built-in class, like `PyDict` or `PyList`

It currently doesn't support using a custom Python class as the parent class for a class defined in Rust.
