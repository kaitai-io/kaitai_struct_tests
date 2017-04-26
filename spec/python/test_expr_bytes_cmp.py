import unittest

from expr_bytes_cmp import ExprBytesCmp

class TestExprBytesCmp(unittest.TestCase):
    def test_expr_bytes_cmp(self):
        r = ExprBytesCmp.from_file("src/fixed_struct.bin")

        self.assertEqual(r.one, b"P")
        self.assertEqual(r.two, b"ACK")

        self.assertEqual(r.is_eq, True)
        self.assertEqual(r.is_ne, False)
        self.assertEqual(r.is_lt, True)
        self.assertEqual(r.is_gt, False)
        self.assertEqual(r.is_le, True)
        self.assertEqual(r.is_ge, False)
        self.assertEqual(r.is_lt2, False)
        self.assertEqual(r.is_gt2, True)
