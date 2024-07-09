# Modify the Rust extension to get the test below to pass
# Do NOT modify the test itself!
from output import fibonacci
import pytest


def test_zero():
    out = fibonacci(0)
    assert out == []


def test_one():
    out = fibonacci(1)
    assert out == [0]


def test_five():
    out = fibonacci(5)
    assert out == [0, 1, 1, 2, 3]


def test_ten():
    out = fibonacci(10)
    assert out == [0, 1, 1, 2, 3, 5, 8, 13, 21, 34]


@pytest.mark.xfail(reason="It makes no sense to ask for the -5th number in Fibonacci's sequence", raises=OverflowError)
def test_negative_numbers():
    fibonacci(-5)
