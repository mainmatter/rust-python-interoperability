# Modify the Rust extension to get the test below to pass
# Do NOT modify the test itself!
import pytest

from outro3 import site_map

def test_site_map():
    discovered_urls = set()
    site_map("https://rust-exercises.com", discovered_urls)
    assert len(discovered_urls) >= 200
