# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from str_encodings_utf16 import _schema

class TestStrEncodingsUtf16(unittest.TestCase):
    def test_str_encodings_utf16(self):
        r = _schema.parse_file('src/str_encodings_utf16.bin')

        self.assertEqual(r.len_be, 12)
        self.assertEqual(r.be_bom_removed.bom, 65279)
        self.assertEqual(r.be_bom_removed.str, u"\u3053\u3093\u306b\u3061\u306f")
        self.assertEqual(r.len_le, 12)
        self.assertEqual(r.le_bom_removed.bom, 65279)
        self.assertEqual(r.le_bom_removed.str, u"\u3053\u3093\u306b\u3061\u306f")