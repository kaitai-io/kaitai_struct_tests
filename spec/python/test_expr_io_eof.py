import unittest

from expr_io_eof import ExprIoEof

class TestExprIoEof(unittest.TestCase):
    def test_expr_io_eof(self):
        r = ExprIoEof.from_file("src/fixed_struct.bin")

        self.assertEqual(r.substream1.one, 1262698832)
        self.assertEqual(hasattr(r.substream1, 'two'), False)

        self.assertEqual(r.substream2.one, 4294914349)
        self.assertEqual(r.substream2.two, 1262698832)
