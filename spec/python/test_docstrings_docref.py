import unittest

from docstrings_docref import DocstringsDocref

class TestDocstringsDocref(unittest.TestCase):
    def test_docstrings_docref(self):
        r = DocstringsDocref.from_file("src/fixed_struct.bin")
