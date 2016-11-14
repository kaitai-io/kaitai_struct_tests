import unittest

from fixed_contents import FixedContents

class TestFixedContents(unittest.TestCase):
    def test_fixed_contents(self):
        r = FixedContents.from_file("src/fixed_struct.bin")
