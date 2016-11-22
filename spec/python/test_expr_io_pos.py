import unittest

from expr_io_pos import ExprIoPos

class TestExprIoPos(unittest.TestCase):
    def test_expr_io_pos(self):
        r = ExprIoPos.from_file("src/expr_io_pos.bin")

        self.assertEqual(r.substream1.my_str, "CURIOSITY")
        self.assertEqual(r.substream1.body, bytearray([0x11, 0x22, 0x33, 0x44]))
        self.assertEqual(r.substream1.number, 0x42)

        self.assertEqual(r.substream2.my_str, "KILLED")
        self.assertEqual(r.substream2.body, b"a cat")
        self.assertEqual(r.substream2.number, 0x67)
