import unittest

from process_xor_value import ProcessXorValue

class TestProcessXorValue(unittest.TestCase):
    def test_process_xor_value(self):
        r = ProcessXorValue.from_file("src/process_xor_1.bin")

        self.assertEquals(r.key, 0xff)
        self.assertEquals(r.buf, "foo bar")
