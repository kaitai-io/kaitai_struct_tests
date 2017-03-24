import unittest

from nav_parent_override import NavParentOverride

class TestNavParentOverride(unittest.TestCase):
    def test_nav_parent_override(self):
        r = NavParentOverride.from_file("src/nav_parent_codes.bin")

        self.assertEqual(r.child_size, 3)
        self.assertEqual(r.child_1.data, bytearray([73, 49, 50]))
        self.assertEqual(r.mediator_2.child_2.data, bytearray([51, 66, 98]))
