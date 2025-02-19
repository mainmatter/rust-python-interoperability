# Modify the Rust extension to get the test below to pass
# Do NOT modify the test itself!
import pytest

from outro2 import Discount, CappedDiscount, SeasonalDiscount, ExpiredDiscount
from datetime import datetime, timedelta, UTC

def test_discount():
    discount = Discount(0.1)
    assert discount.percentage == pytest.approx(0.1)
    assert discount.apply(100) == pytest.approx(90)

def test_capped_discount():
    discount = CappedDiscount(0.6, 50)
    assert discount.percentage == pytest.approx(0.6)
    assert discount.cap == pytest.approx(50)
    assert discount.apply(100) == pytest.approx(50)
    assert discount.apply(50) == pytest.approx(20)

def test_seasonal_discount():
    current = datetime.now(UTC)
    from_ = current - timedelta(days=2)
    to = current + timedelta(days=2)
    discount = SeasonalDiscount(0.1, from_, to)
    assert discount.percentage == pytest.approx(0.1)
    assert discount.from_ == from_
    assert discount.to == to
    assert discount.apply(100) == pytest.approx(90)

# Validation tests
def test_discount_validation_negative_percentage():
    with pytest.raises(ValueError) as exc_info:
        Discount(-0.1)
    assert str(exc_info.value) == "Percentage must be between 0 and 1"

def test_discount_validation_percentage_too_high():
    with pytest.raises(ValueError) as exc_info:
        Discount(1.1)
    assert str(exc_info.value) == "Percentage must be between 0 and 1"

def test_discount_validation_negative_percentage_setter():
    discount = Discount(0.1)
    with pytest.raises(ValueError) as exc_info:
        discount.percentage = -0.1
    assert str(exc_info.value) == "Percentage must be between 0 and 1"

def test_discount_validation_percentage_too_high_setter():
    discount = Discount(0.1)
    with pytest.raises(ValueError) as exc_info:
        discount.percentage = 1.1
    assert str(exc_info.value) == "Percentage must be between 0 and 1"

def test_capped_discount_validation_negative_percentage():
    with pytest.raises(ValueError) as exc_info:
        CappedDiscount(-0.1, 50)
    assert str(exc_info.value) == "Percentage must be between 0 and 1"

def test_capped_discount_validation_percentage_too_high():
    with pytest.raises(ValueError) as exc_info:
        CappedDiscount(1.1, 50)
    assert str(exc_info.value) == "Percentage must be between 0 and 1"

def test_capped_discount_validation_negative_cap():
    with pytest.raises(ValueError) as exc_info:
        CappedDiscount(0.1, -50)
    assert str(exc_info.value) == "Cap must be a positive number"

def test_capped_discount_validation_zero_cap():
    with pytest.raises(ValueError) as exc_info:
        CappedDiscount(0.1, 0)
    assert str(exc_info.value) == "Cap must be a positive number"

def test_capped_discount_validation_negative_cap_setter():
    discount = CappedDiscount(0.1, 50)
    with pytest.raises(ValueError) as exc_info:
        discount.cap = -50
    assert str(exc_info.value) == "Cap must be a positive number"

def test_capped_discount_validation_zero_cap_setter():
    discount = CappedDiscount(0.1, 50)
    with pytest.raises(ValueError) as exc_info:
        discount.cap = 0
    assert str(exc_info.value) == "Cap must be a positive number"

def test_seasonal_discount_validation_to_before_from():
    with pytest.raises(ValueError) as exc_info:
        SeasonalDiscount(0.1, datetime.now(UTC), datetime.now(UTC) - timedelta(days=1))
    assert str(exc_info.value) == "`from_` date must be before `to` date"

def test_seasonal_discount_validation_same_dates():
    now = datetime.now(UTC)
    with pytest.raises(ValueError) as exc_info:
        SeasonalDiscount(0.1, now, now)
    assert str(exc_info.value) == "`from_` date must be before `to` date"

def test_seasonal_discount_validation_negative_percentage():
    with pytest.raises(ValueError) as exc_info:
        SeasonalDiscount(-0.1, datetime.now(UTC), datetime.now(UTC) + timedelta(days=1))
    assert str(exc_info.value) == "Percentage must be between 0 and 1"

def test_seasonal_discount_validation_expired():
    with pytest.raises(ExpiredDiscount) as exc_info:
        SeasonalDiscount(0.5, datetime.now(UTC) - timedelta(days=2), datetime.now(UTC) - timedelta(days=1))
    assert str(exc_info.value) == "The discount is no longer active"

def test_seasonal_discount_validation_from_after_to_setter():
    discount = SeasonalDiscount(0.5, datetime.now(UTC) + timedelta(days=1), datetime.now(UTC) + timedelta(days=2))
    with pytest.raises(ValueError) as exc_info:
        discount.from_ = datetime.now(UTC) + timedelta(days=3)
    assert str(exc_info.value) == "`from_` date must be before `to` date"
