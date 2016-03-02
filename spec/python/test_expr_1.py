# coding: utf-8
import unittest

from expr_1 import Expr1

class TestExpr1(unittest.TestCase):
    def test_expr1(self):
        r = Expr1.from_file("src/str_encodings.bin")

        self.assertEquals(r.len_of_1, 10)
        self.assertEquals(r.len_of_1_mod, 8)
        self.assertEquals(r.str1, "Some ASC")
        self.assertEquals(r.str1_len, 8)
