import unittest

from imports0 import Imports0

class TestImports0(unittest.TestCase):
    def test_imports0(self):
        r = Imports0.from_file("src/fixed_struct.bin")

        self.assertEqual(r.two, 0x50)
        self.assertEqual(r.hw.one, 0x41)
        self.assertEqual(r.hw_one, 0x41)
