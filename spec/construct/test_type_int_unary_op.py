# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from type_int_unary_op import _schema

class TestTypeIntUnaryOp(unittest.TestCase):
    def test_type_int_unary_op(self):
        r = _schema.parse_file('src/fixed_struct.bin')
        self.assertEqual(r.value_s2, 16720)
        self.assertEqual(r.value_s8, 4706543082108963651)
        self.assertEqual(r.unary_s2, -16720)
        self.assertEqual(r.unary_s8, -4706543082108963651)
