// TODO: Add a `total` method to the `ShoppingOrder` class that returns the total cost of the order.
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyInt;

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
    /// Raises `ValueError` if:
    /// - `name` is an empty string or just whitespace.
    /// - `price` is zero or negative.
    /// - `quantity` is zero or negative.
    fn new(name: String, price: Bound<'_, PyInt>, quantity: Bound<'_, PyInt>) -> PyResult<Self> {
        let price = price
            .extract()
            .map_err(|_| PyValueError::new_err("Price must be an non-negative integer"))?;
        if price == 0 {
            return Err(PyValueError::new_err("Price cannot be zero"));
        }

        let quantity = quantity
            .extract()
            .map_err(|_| PyValueError::new_err("Quantity must be an non-negative integer"))?;
        if quantity == 0 {
            return Err(PyValueError::new_err("Quantity cannot be zero"));
        }

        if name.trim().is_empty() {
            return Err(PyValueError::new_err("Name cannot be empty"));
        }

        Ok(ShoppingOrder {
            name,
            price,
            quantity,
        })
    }

    /// Returns the total cost of the order.
    fn total(&self) -> u64 {
        self.price * self.quantity
    }
}

#[pymodule]
fn methods(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<ShoppingOrder>()?;
    Ok(())
}
