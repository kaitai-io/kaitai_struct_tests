import unittest

from process_xor4_const import ProcessXor4Const

class TestProcessXor4Const(unittest.TestCase):
    def test_process_xor4_const(self):
        r = ProcessXor4Const.from_file("src/process_xor_4.bin")

        self.assertEquals(r.key, bytearray([0xec, 0xbb, 0xa3, 0x14]))
        self.assertEquals(r.buf, "foo bar")
