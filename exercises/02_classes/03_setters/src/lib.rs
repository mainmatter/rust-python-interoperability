// TODO: Every time either `name` or `price` is accessed, increment the `n_visits` field in `Item` by one.
use pyo3::prelude::*;

#[pyclass]
struct Item {
    name: String,
    price: u64,
    #[pyo3(get)]
    n_visits: u64,
}

#[pymethods]
impl Item {
    #[new]
    fn new(name: String, price: u64) -> Self {
        Item {
            name,
            price,
            n_visits: 0,
        }
    }
}

#[pymodule]
fn setters(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Item>()?;
    Ok(())
}
