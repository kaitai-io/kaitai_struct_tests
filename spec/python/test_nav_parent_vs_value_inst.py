import unittest

from nav_parent_vs_value_inst import NavParentVsValueInst

class TestNavParentVsValueInst(unittest.TestCase):
    def test_nav_parent_vs_value_inst(self):
        r = NavParentVsValueInst.from_file("src/term_strz.bin")

        self.assertEqual(r.s1, "foo")
