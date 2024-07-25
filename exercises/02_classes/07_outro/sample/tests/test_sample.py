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
def test_discount_validation():
    try:
        Discount(-0.1)
    except ValueError as e:
        assert str(e) == "Percentage must be between 0 and 1"

    try:
        Discount(1.1)
    except ValueError as e:
        assert str(e) == "Percentage must be between 0 and 1"

    try:
        discount = Discount(0.1)
        discount.percentage = -0.1
    except ValueError as e:
        assert str(e) == "Percentage must be between 0 and 1"

    try:
        discount = Discount(0.1)
        discount.percentage = 1.1
    except ValueError as e:
        assert str(e) == "Percentage must be between 0 and 1"

def test_capped_discount_validation():
    try:
        CappedDiscount(-0.1, 50)
    except ValueError as e:
        assert str(e) == "Percentage must be between 0 and 1"

    try:
        CappedDiscount(1.1, 50)
    except ValueError as e:
        assert str(e) == "Percentage must be between 0 and 1"

    try:
        CappedDiscount(0.1, -50)
    except ValueError as e:
        assert str(e) == "Cap must be a positive number"

    try:
        CappedDiscount(0.1, 0)
    except ValueError as e:
        assert str(e) == "Cap must be a positive number"

    try:
        discount = CappedDiscount(0.1, 50)
        discount.cap = -50
    except ValueError as e:
        assert str(e) == "Cap must be a positive number"

    try:
        discount = CappedDiscount(0.1, 50)
        discount.cap = 0
    except ValueError as e:
        assert str(e) == "Cap must be a positive number"

def test_seasonal_discount_validation():
    try:
        SeasonalDiscount(0.1, datetime.now(UTC), datetime.now(UTC) - timedelta(days=1))
    except ValueError as e:
        assert str(e) == "`from_` date must be before `to` date"

    try:
        now = datetime.now(UTC)
        SeasonalDiscount(0.1, now, now)
    except ValueError as e:
        assert str(e) == "`from_` date must be before `to` date"

    try:
        SeasonalDiscount(-0.1, datetime.now(UTC), datetime.now(UTC) + timedelta(days=1))
    except ValueError as e:
        assert str(e) == "Percentage must be between 0 and 1"

    try:
        SeasonalDiscount(0.5, datetime.now(UTC) - timedelta(days=2), datetime.now(UTC) - timedelta(days=1))
    except ExpiredDiscount as e:
        assert str(e) == "The discount is no longer active"

    try:
        discount = SeasonalDiscount(0.5, datetime.now(UTC) + timedelta(days=1), datetime.now(UTC) + timedelta(days=2))
        discount.from_ = datetime.now(UTC) + timedelta(days=3)
    except ValueError as e:
        assert str(e) == "`from_` date must be before `to` date"

