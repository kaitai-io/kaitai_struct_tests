import unittest

from docstrings import Docstrings

class TestDocstrings(unittest.TestCase):
    def test_docstrings(self):
        r = Docstrings.from_file("src/fixed_struct.bin")
