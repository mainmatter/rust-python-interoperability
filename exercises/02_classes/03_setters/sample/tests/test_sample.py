# Modify the Rust extension to get the test below to pass
# Do NOT modify the test itself!
from setters import Item
import pytest

def test_setters():
    item = Item("Cart", 10)
    assert item.n_visits == 0

    _ = item.name
    assert item.n_visits == 1

    _ = item.price
    assert item.n_visits == 2

    _ = item.price
    assert item.n_visits == 3