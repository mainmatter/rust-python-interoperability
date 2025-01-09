use pyo3::prelude::*;

#[pyclass(frozen)]
struct Point {
    x: i32,
    y: i32,
}

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

#[pymodule]
fn immutable(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(print_point, m)?)?;
    Ok(())
}
