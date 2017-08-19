import unittest

from js_signed_right_shift import JsSignedRightShift

class TestJsSignedRightShift(unittest.TestCase):
    def test_js_signed_right_shift(self):
        r = JsSignedRightShift.from_file("src/fixed_struct.bin")

        self.assertEqual(r.should_be_40000000, 0x40000000)
        self.assertEqual(r.should_be_a00000, 0xa00000)
