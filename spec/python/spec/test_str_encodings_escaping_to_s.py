import unittest
from testformats.str_encodings_escaping_to_s import StrEncodingsEscapingToS

class TestStrEncodingsEscapingToS(unittest.TestCase):
    def test_str_encodings_escaping_to_s(self):
        with StrEncodingsEscapingToS.from_file('src/str_encodings.bin') as r:
            with self.assertRaises(LookupError) as cm:
                r.str1
            self.assertEqual(str(cm.exception), 'unknown encoding: {}'.format("ASCII\\\\x"))

            with self.assertRaises(LookupError) as cm:
                r.str2
            self.assertEqual(str(cm.exception), 'unknown encoding: {}'.format("UTF-8\\'x"))

            with self.assertRaises(LookupError) as cm:
                r.str3
            self.assertEqual(str(cm.exception), 'unknown encoding: {}'.format("SJIS\\\"x"))

            with self.assertRaises(LookupError) as cm:
                r.str4
            self.assertEqual(str(cm.exception), 'unknown encoding: {}'.format("IBM437\\nx"))
