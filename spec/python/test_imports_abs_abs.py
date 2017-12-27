import unittest

from imports_abs_abs import ImportsAbsAbs

class TestImportsAbsAbs(unittest.TestCase):
    def test_imports_abs_abs(self):
        r = ImportsAbsAbs.from_file("src/fixed_struct.bin")

        self.assertEqual(r.one, 0x50)
        self.assertEqual(r.two.one, 0x41)
        self.assertEqual(r.two.two.one, 0x43)
