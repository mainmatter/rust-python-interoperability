# Modify the Rust extension to get the test below to pass
# Do NOT modify the test itself!
import pytest

from minimize import compute_prime_factors

def test_compute_prime_factors():
    numbers = [387, 2, 75, 452, 562672865058083521]
    number2prime_factors = compute_prime_factors(numbers)
    assert number2prime_factors == {
        387: [3, 43],
        2: [2],
        75: [3, 5],
        452: [2, 113],
        562672865058083521: [7, 11483119695062929]
    }
