# Modify the Rust extension to get the test below to pass
# Do NOT modify the test itself!
from static_methods import Discount

def test_discount():
    assert Discount(0.4).percentage == 0.4

    discount = Discount.default()
    assert discount.percentage == 0.1