# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from js_signed_right_shift import _schema

class TestJsSignedRightShift(unittest.TestCase):
    def test_js_signed_right_shift(self):
        r = _schema.parse_file('src/fixed_struct.bin')
        self.assertEqual(r.should_be_40000000, 1073741824)
        self.assertEqual(r.should_be_a00000, 10485760)
