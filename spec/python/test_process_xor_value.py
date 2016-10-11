import unittest

from process_xor_value import ProcessXorValue

class TestProcessXorValue(unittest.TestCase):
    def test_process_xor_value(self):
        with ProcessXorValue.from_file("src/process_xor_1.bin") as r:

            self.assertEqual(r.key, 0xff)
            self.assertEqual(r.buf, b"foo bar")
