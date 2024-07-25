use chrono::{DateTime, Utc};
use pyo3::create_exception;
// TODO: Define a base class named `Discount`, with a `percentage` attribute.
//  It should be possible to access the `percentage` attribute of a `Discount`.
//  It should also be possible to modify the `percentage` attribute of a `Discount`.
//  It must be enforced that the `percentage` attribute is a float between 0. and 1.
//  Then define two subclasses:
//  - `SeasonalDiscount` that inherits from `Discount` with two additional attributes, `to` and `from_`.
//    `from_` is a datetime object that represents the start of the discount period.
//    `to` is a datetime object that represents the end of the discount period.
//     Both `from_` and `to` should be accessible and modifiable.
//     The class should enforce that `from` is before `to`.
//  - `CappedDiscount` that inherits from `Discount` with an additional attribute `cap`.
//    `cap` is a float that represents the maximum discount (in absolute value) that can be applied.
//    It should be possible to access and modify the `cap` attribute.
//    The class should enforce that `cap` is a non-zero positive float.
//
// All classes should have a method named `apply` that takes a price (float) as input and
// returns the discounted price.
// `SeasonalDiscount` should raise an `ExpiredDiscount` exception if `apply` is called but
// the current date is outside the discount period.
use pyo3::prelude::*;

#[pyclass(subclass)]
struct Discount {
    #[pyo3(get)]
    percentage: f64,
}

#[pymethods]
impl Discount {
    #[new]
    fn new(percentage: f64) -> PyResult<Self> {
        validate_percentage(percentage)?;
        Ok(Discount { percentage })
    }

    #[setter]
    fn percentage(&mut self, p: f64) -> PyResult<()> {
        validate_percentage(p)?;
        self.percentage = p;
        Ok(())
    }

    fn apply(&self, price: f64) -> f64 {
        price * (1. - self.percentage)
    }
}

fn validate_percentage(p: f64) -> PyResult<()> {
    if p < 0. || p > 1. {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Percentage must be between 0 and 1",
        ));
    }
    Ok(())
}

#[pyclass(extends = Discount)]
struct SeasonalDiscount {
    #[pyo3(get)]
    from_: DateTime<Utc>,
    #[pyo3(get)]
    to: DateTime<Utc>,
}

create_exception!(outro2, ExpiredDiscount, pyo3::exceptions::PyException);

#[pymethods]
impl SeasonalDiscount {
    #[new]
    fn new(
        percentage: f64,
        from_: DateTime<Utc>,
        to: DateTime<Utc>,
    ) -> PyResult<PyClassInitializer<Self>> {
        validate_range(from_, to)?;
        let discount = Discount::new(percentage)?;
        let seasonal = SeasonalDiscount { from_, to };
        Ok(PyClassInitializer::from(discount).add_subclass(seasonal))
    }

    fn apply(self_: PyRef<'_, Self>, price: f64) -> PyResult<f64> {
        let now = Utc::now();
        if now < self_.from_ || now > self_.to {
            return Err(ExpiredDiscount::new_err("The discount is no longer active"));
        }
        Ok(self_.as_super().apply(price))
    }

    #[setter]
    fn from_(&mut self, from_: DateTime<Utc>) -> PyResult<()> {
        validate_range(from_, self.to)?;
        self.from_ = from_;
        Ok(())
    }

    #[setter]
    fn to(&mut self, to: DateTime<Utc>) -> PyResult<()> {
        validate_range(self.from_, to)?;
        self.to = to;
        Ok(())
    }
}

fn validate_range(from_: DateTime<Utc>, to: DateTime<Utc>) -> PyResult<()> {
    if from_ >= to {
        Err(pyo3::exceptions::PyValueError::new_err(
            "`from_` date must be before `to` date",
        ))
    } else {
        Ok(())
    }
}

#[pyclass(extends = Discount)]
struct CappedDiscount {
    #[pyo3(get)]
    cap: f64,
}

#[pymethods]
impl CappedDiscount {
    #[new]
    fn new(percentage: f64, cap: f64) -> PyResult<PyClassInitializer<Self>> {
        validate_cap(cap)?;
        let discount = Discount::new(percentage)?;
        let capped = CappedDiscount { cap };
        Ok(PyClassInitializer::from(discount).add_subclass(capped))
    }

    fn apply(self_: PyRef<'_, Self>, price: f64) -> f64 {
        let discounted = self_.as_super().apply(price);
        if price - discounted > self_.cap {
            price - self_.cap
        } else {
            discounted
        }
    }

    #[setter]
    fn cap(&mut self, cap: f64) -> PyResult<()> {
        validate_cap(cap)?;
        self.cap = cap;
        Ok(())
    }
}

fn validate_cap(cap: f64) -> PyResult<()> {
    if cap <= 0. {
        Err(pyo3::exceptions::PyValueError::new_err(
            "Cap must be a positive number",
        ))
    } else {
        Ok(())
    }
}

#[pymodule]
fn outro2(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Discount>()?;
    m.add_class::<SeasonalDiscount>()?;
    m.add_class::<CappedDiscount>()?;
    m.add("ExpiredDiscount", m.py().get_type::<ExpiredDiscount>())?;
    Ok(())
}
