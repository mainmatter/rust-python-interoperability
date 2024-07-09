# Modify the Rust extension to get the test below to pass
# Do NOT modify the test itself!
from gil import print_number_list
import pytest

def test_empty_list(capfd):
    print_number_list([])
    out, _ = capfd.readouterr()
    assert out == ""

def test_single_element_list(capfd):
    print_number_list([1])
    out, _ = capfd.readouterr()
    assert out == "1\n"

def test_multiple_element_list(capfd):
    print_number_list([1, 2, 3, 4, 5])
    out, _ = capfd.readouterr()
    assert out == "1\n2\n3\n4\n5\n"

@pytest.mark.xfail(reason="Negative numbers are not supported")
def test_negative_numbers():
    print_number_list([-1, -2, -3, -4, -5])

@pytest.mark.xfail(reason="Numbers larger than u64::MAX are not supported")
def test_larger_than_u64():
    print_number_list([2**64, 2**64 + 1, 2**64 + 2, 2**64 + 3, 2**64 + 4])
