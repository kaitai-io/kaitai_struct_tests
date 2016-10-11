import unittest

from process_xor4_value import ProcessXor4Value

class TestProcessXor4Value(unittest.TestCase):
    def test_process_xor4_value(self):
        with ProcessXor4Value.from_file("src/process_xor_4.bin") as r:

            self.assertEqual(r.key, bytearray([0xec, 0xbb, 0xa3, 0x14]))
            self.assertEqual(r.buf, b"foo bar")
