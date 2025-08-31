import unittest

from testformats.expr_io_eof_bits import ExprIoEofBits

class TestExprIoEofBits(unittest.TestCase):
    def test_expr_io_eof_bits(self):
        with ExprIoEofBits.from_file('src/nav_parent_switch.bin') as r:
            r._read()

            self.assertEqual(r.foo, 5167)
            self.assertEqual(r.bar, 15)
            self.assertFalse(hasattr(r, 'baz'))
            self.assertEqual(r.align, b'')
            self.assertFalse(hasattr(r, 'qux'))
