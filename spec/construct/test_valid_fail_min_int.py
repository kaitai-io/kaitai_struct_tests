# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
import kaitaistruct

from valid_fail_min_int import _schema

class TestValidFailMinInt(unittest.TestCase):
    def test_valid_fail_min_int(self):
        with self.assertRaises(kaitaistruct.ValidationLessThanError):
            r = _schema.parse_file('src/fixed_struct.bin')
