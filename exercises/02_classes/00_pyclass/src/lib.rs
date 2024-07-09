use pyo3::prelude::*;

// TODO: Create a new Python class named `ShoppingOrder` with the following attributes:
//  - `price` (positive integer)
//  - `quantity` (positive integer)
//  - `name` (string)
//  Expose a `new_order` function to create a new `ShoppingOrder` instance.
//  It shouldn't be possible to modify the name or the price after the object is created.


#[pymodule]
fn pyclass(m: &Bound<'_, PyModule>) -> PyResult<()> {
    Ok(())
}
