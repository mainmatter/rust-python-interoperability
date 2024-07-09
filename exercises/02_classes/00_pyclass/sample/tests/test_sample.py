# Modify the Rust extension to get the test below to pass
# Do NOT modify the test itself!
from pyclass import ShoppingOrder, new_order
import pytest

def test_new_order():
    order = new_order("apple", 10, 5)
    assert order.name == "apple"
    assert order.price == 10
    assert order.quantity == 5

    order.quantity = 10
    assert order.quantity == 10

    with pytest.raises(AttributeError):
        order.name = "banana"

    with pytest.raises(AttributeError):
        order.price = 20

