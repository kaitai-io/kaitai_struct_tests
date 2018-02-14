import unittest

from process_custom import ProcessCustom

class MyCustomFx(object):
    def __init__(self, key, flag, some_array):
        self.key = key

    def decode(self, data):
        data

class TestProcessCustom(unittest.TestCase):
    def test_process_custom(self):
        r = ProcessCustom.from_file("src/process_rotate.bin")

        self.assertEqual(r.buf1, b"\x10\xb3\x94\x94\xf4")
        self.assertEqual(r.buf2, b"\x5f\xba\x7b\x93\x63\x23\x5f")
        self.assertEqual(r.buf3, b"\x29\x33\xb1\x38\xb1")
