import unittest
from testformats.expr_io_eof import ExprIoEof

class TestExprIoEof(unittest.TestCase):
    def test_expr_io_eof(self):
        with ExprIoEof.from_file('src/fixed_struct.bin') as r:
            self.assertEqual(r.substream1.one, 1262698832)
            self.assertFalse(hasattr(r.substream1, 'two'))
            self.assertEqual(r.substream2.one, 4294914349)
            self.assertEqual(r.substream2.two, 1262698832)
