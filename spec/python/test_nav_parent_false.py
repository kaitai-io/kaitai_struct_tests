import unittest

from nav_parent_false import NavParentFalse

class TestNavParentFalse(unittest.TestCase):
    def test_nav_parent_false(self):
        r = NavParentFalse.from_file("src/nav_parent_codes.bin")

        self.assertEqual(r.child_size, 3)
        self.assertEqual(r.element_a.foo.code, 73)
        self.assertEqual(r.element_a.foo.more, b"123")
        self.assertEqual(r.element_a.bar.foo.code, 66)
        self.assertEqual(r.element_b.foo.code, 98)
