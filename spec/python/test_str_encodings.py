# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from str_encodings import StrEncodings

class TestStrEncodings(unittest.TestCase):
    def test_str_encodings(self):
        with StrEncodings.from_file('src/str_encodings.bin') as r:

            self.assertEqual(r.str1, u"Some ASCII")
            self.assertEqual(r.str2, u"\u3053\u3093\u306b\u3061\u306f")
            self.assertEqual(r.str3, u"\u3053\u3093\u306b\u3061\u306f")
            self.assertEqual(r.str4, u"\u2591\u2592\u2593")
