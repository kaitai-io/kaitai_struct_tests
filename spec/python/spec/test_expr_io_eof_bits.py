import unittest
import kaitaistruct

from testformats.expr_io_eof_bits import ExprIoEofBits

class TestExprIoEofBits(unittest.TestCase):
    def test_expr_io_eof_bits(self):
        with ExprIoEofBits.from_file('src/nav_parent_switch.bin') as r:
            with self.assertRaisesRegexp(EOFError, u"^requested 1 bytes, but only 0 bytes available$"):
                r._read()
            self.assertEqual(r.foo, 5167)
            self.assertEqual(r.bar, 15)
            self.assertEqual(r.assert_io_eof_before_baz, b"")
            self.assertFalse(hasattr(r, 'baz'))
            self.assertFalse(hasattr(r, 'assert_io_eof_after_baz'))
