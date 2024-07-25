use pyo3::prelude::*;

// TODO: Create a new Python class named `ShoppingOrder` with the following attributes:
//  - `price` (positive integer)
//  - `quantity` (positive integer)
//  - `name` (string)
//  Expose a `new_order` function to create a new `ShoppingOrder` instance.
//  It shouldn't be possible to modify the name or the price after the object is created.

#[pyclass]
struct ShoppingOrder {
    #[pyo3(get)]
    price: u64,
    #[pyo3(get, set)]
    quantity: u64,
    #[pyo3(get)]
    name: String,
}

#[pyfunction]
fn new_order(name: String, price: u64, quantity: u64) -> ShoppingOrder {
    ShoppingOrder {
        price,
        quantity,
        name,
    }
}

#[pymodule]
fn pyclass(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ShoppingOrder>()?;
    m.add_function(wrap_pyfunction!(new_order, m)?)?;
    Ok(())
}
