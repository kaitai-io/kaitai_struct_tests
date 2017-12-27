import unittest

from imports_abs_rel import ImportsAbsRel

class TestImportsAbsRel(unittest.TestCase):
    def test_imports_abs_rel(self):
        r = ImportsAbsRel.from_file("src/fixed_struct.bin")

        self.assertEqual(r.one, 0x50)
        self.assertEqual(r.two.one, 0x41)
        self.assertEqual(r.two.two.one, 0x43)
