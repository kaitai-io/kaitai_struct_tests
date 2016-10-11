# coding: utf-8
import unittest

from expr_1 import Expr1

class TestExpr1(unittest.TestCase):
    def test_expr_1(self):
        with Expr1.from_file("src/str_encodings.bin") as r:

            self.assertEqual(r.len_of_1, 10)
            self.assertEqual(r.len_of_1_mod, 8)
            self.assertEqual(r.str1, "Some ASC")
            self.assertEqual(r.str1_len, 8)
