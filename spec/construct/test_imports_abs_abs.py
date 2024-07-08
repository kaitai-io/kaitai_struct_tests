# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from imports_abs_abs import _schema

class TestImportsAbsAbs(unittest.TestCase):
    def test_imports_abs_abs(self):
        r = _schema.parse_file('src/fixed_struct.bin')

        self.assertEqual(r.one, 80)
        self.assertEqual(r.two.one, 65)
        self.assertEqual(r.two.two.one, 67)
