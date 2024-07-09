# Modify the Rust extension to get the test below to pass
# Do NOT modify the test itself!
from methods import ShoppingOrder
import pytest

def test_total():
    order = ShoppingOrder("apple", 10, 5)
    assert order.total() == 50

    order.quantity = 10
    assert order.total() == 100

    order.quantity = 2
    assert order.total() == 20

def test_visibility():
    order = ShoppingOrder("apple", 10, 5)
    assert order.name == "apple"
    assert order.price == 10
    assert order.quantity == 5

    order.quantity = 10
    assert order.quantity == 10

    with pytest.raises(AttributeError):
        order.name = "banana"

    with pytest.raises(AttributeError):
        order.price = 20

@pytest.mark.xfail(reason="Negative prices are not supported", raises=ValueError)
def test_negative_price():
    ShoppingOrder("apple", -10, 5)

@pytest.mark.xfail(reason="Negative quantities are not supported", raises=ValueError)
def test_negative_quantity():
    ShoppingOrder("apple", 10, -5)

@pytest.mark.xfail(reason="Zero quantities are not supported", raises=ValueError)
def test_zero_quantity():
    ShoppingOrder("apple", 10, 0)

@pytest.mark.xfail(reason="Zero prices are not supported", raises=ValueError)
def test_zero_price():
    ShoppingOrder("apple", 0, 5)

@pytest.mark.xfail(reason="Name can't be empty", raises=ValueError)
def test_empty_name():
    ShoppingOrder("", 10, 5)

@pytest.mark.xfail(reason="Name can't be just whitespace", raises=ValueError)
def test_whitespace_name():
    ShoppingOrder("  ", 10, 5)

