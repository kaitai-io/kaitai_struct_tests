import unittest

from type_int_unary_op import TypeIntUnaryOp

class TestTypeIntUnaryOp(unittest.TestCase):
    def test_type_int_unary_op(self):
        r = TypeIntUnaryOp.from_file("src/fixed_struct.bin")

        self.assertEqual(r.value_s2, 0x4150)
        self.assertEqual(r.value_s8, 0x4150ffff312d4b43)
        self.assertEqual(r.unary_s2, -0x4150)
        self.assertEqual(r.unary_s8, -0x4150ffff312d4b43)
