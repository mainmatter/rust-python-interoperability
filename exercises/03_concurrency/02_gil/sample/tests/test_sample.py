# Modify the Rust extension to get the test below to pass
# Do NOT modify the test itself!
import pytest

from gil2 import word_count

def test_word_count_single_process():
    text = "hello world"
    assert word_count(text, 1) == 2


def test_word_count_multiple_processes():
    text = "hello world"
    assert word_count(text, 2) == 2


def test_word_count_multiple_processes_long_text():
    text = "hello world " * 1000
    assert word_count(text, 2) == 2000


def test_more_processes_than_words():
    text = "hello world"
    assert word_count(text, 10) == 2
