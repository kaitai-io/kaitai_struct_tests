import unittest

from nav_parent_false2 import NavParentFalse2

class TestNavParentFalse2(unittest.TestCase):
    def test_nav_parent_false2(self):
        r = NavParentFalse2.from_file("src/fixed_struct.bin")

        self.assertEqual(r.parentless.foo, 80)
