# Modify the Rust extension to get the test below to pass
# Do NOT modify the test itself!
import pytest

from immutable import compute_area, Rectangle

def test_compute_prime_factors():
    s = Rectangle(10, 12)
    assert compute_area(s) == 120
