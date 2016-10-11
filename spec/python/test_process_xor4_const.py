import unittest

from process_xor4_const import ProcessXor4Const

class TestProcessXor4Const(unittest.TestCase):
    def test_process_xor4_const(self):
        with ProcessXor4Const.from_file("src/process_xor_4.bin") as r:

            self.assertEqual(r.key, bytearray([0xec, 0xbb, 0xa3, 0x14]))
            self.assertEqual(r.buf, b"foo bar")
