# Modify the Rust extension to get the test below to pass
# Do NOT modify the test itself!
from outro1 import max_k
import pytest


# Tests to check the behaviour on the edge cases

def test_empty_list():
    assert max_k([], 0) == []

def test_single_element_list():
    assert max_k([1], 1) == [1]

def test_non_empty_with_0():
    assert max_k([1], 0) == []

def test_multiple_element_list():
    assert max_k([1, 2, 3, 4, 5], 5) == [5, 4, 3, 2, 1]

def test_k_larger_than_list():
    with pytest.raises(ValueError):
        max_k([1, 2, 3, 4, 5], 10)

def test_k_smaller_than_list():
    assert max_k([1, 2, 3, 4, 5], 3) == [5, 4, 3]

def test_negative_numbers():
    with pytest.raises(TypeError):
        max_k([-1, -2, -3, -4, -5], 5)

def test_larger_than_u64():
    assert max_k([2**64, 2**64 + 1], 1) == [2**64 + 1]