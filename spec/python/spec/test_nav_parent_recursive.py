import unittest
from testformats.nav_parent_recursive import NavParentRecursive

class TestNavParentRecursive(unittest.TestCase):
    def test_nav_parent_recursive(self):
        with NavParentRecursive.from_file('src/enum_negative.bin') as r:
            self.assertEqual(r.value, 255)
            self.assertEqual(r.next.value, 1)
            self.assertEqual(r.next.parent_value, 255)
            self.assertFalse(hasattr(r.next, 'next'))
