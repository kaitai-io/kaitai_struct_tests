import unittest

from expr_3 import Expr3

class TestExpr3(unittest.TestCase):
    def test_expr_3(self):
        r = Expr3.from_file("src/fixed_struct.bin")

        self.assertEquals(r.one, 80)
        self.assertEquals(r.two, "ACK")

        self.assertEquals(r.three, "@ACK")
        self.assertEquals(r.four, "_ACK_")
        self.assertEquals(r.is_str_eq, True)
        self.assertEquals(r.is_str_ne, False)
        self.assertEquals(r.is_str_lt, True)
        self.assertEquals(r.is_str_gt, False)
        self.assertEquals(r.is_str_le, True)
        self.assertEquals(r.is_str_ge, False)
        self.assertEquals(r.is_str_lt2, True)
