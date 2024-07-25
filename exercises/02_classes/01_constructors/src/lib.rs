use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyInt;
// TODO: Add a `__new__` constructor to the `ShoppingOrder` class that takes the following arguments:
//  - `name` (non-empty string)
//  - `price` (non-zero integer)
//  - `quantity` (non-zero integer)
//  The constructor should raise a `ValueError` if any of the arguments are invalid.

#[pyclass]
struct ShoppingOrder {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    price: u64,
    #[pyo3(get, set)]
    quantity: u64,
}

#[pymethods]
impl ShoppingOrder {
    #[new]
    fn new(name: String, price: Bound<'_, PyInt>, quantity: Bound<'_, PyInt>) -> PyResult<Self> {
        if name.trim().is_empty() {
            return Err(PyValueError::new_err("name cannot be empty"));
        }
        let price: u64 = price
            .extract()
            .map_err(|_| PyValueError::new_err("price must be non-negative"))?;
        if price == 0 {
            return Err(PyValueError::new_err("price cannot be zero"));
        }
        let quantity: u64 = quantity
            .extract()
            .map_err(|_| PyValueError::new_err("quantity must be non-negative"))?;
        if quantity == 0 {
            return Err(PyValueError::new_err("quantity cannot be zero"));
        }
        Ok(ShoppingOrder {
            name,
            price,
            quantity,
        })
    }
}

#[pymodule]
fn constructors(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ShoppingOrder>()?;
    Ok(())
}
