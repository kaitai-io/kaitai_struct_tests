# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from nav_parent_recursive import _schema

class TestNavParentRecursive(unittest.TestCase):
    def test_nav_parent_recursive(self):
        r = _schema.parse_file('src/enum_negative.bin')

        self.assertEqual(r.value, 255)
        self.assertEqual(r.next.value, 1)
        self.assertEqual(r.next.parent_value, 255)
        self.assertIsNone(r.next.next)
