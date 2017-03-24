import unittest

from imports_abs import ImportsAbs

class TestImportsAbs(unittest.TestCase):
    def test_imports_abs(self):
        r = ImportsAbs.from_file("src/fixed_struct.bin")

        self.assertEqual(r.len.value, 80)
        self.assertEqual(len(r.body), 80)
