import unittest
from testformats.nav_root_recursive import NavRootRecursive

class TestNavRootRecursive(unittest.TestCase):
    def test_nav_root_recursive(self):
        with NavRootRecursive.from_file('src/enum_negative.bin') as r:
            self.assertEqual(r.value, 255)
            self.assertEqual(r.next.value, 1)
            self.assertEqual(r.next.root_value, 255)
            self.assertFalse(hasattr(r.next, 'next'))
