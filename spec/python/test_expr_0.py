# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from expr_0 import Expr0

class TestExpr0(unittest.TestCase):
    def test_expr_0(self):
        with Expr0.from_file('src/str_encodings.bin') as r:

            self.assertEqual(r.must_be_f7, 247)
            self.assertEqual(r.must_be_abc123, u"abc123")
