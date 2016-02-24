import unittest

from process_xor_const import ProcessXorConst

class TestProcessXorConst(unittest.TestCase):
    def test_process_xor_const(self):
        r = ProcessXorConst.from_file("src/process_xor_1.bin")

        self.assertEquals(r.key, 0xff)
        self.assertEquals(r.buf, "foo bar")
