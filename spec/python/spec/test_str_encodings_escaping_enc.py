import unittest
from testformats.str_encodings_escaping_enc import StrEncodingsEscapingEnc

class TestStrEncodingsEscapingEnc(unittest.TestCase):
    def test_str_encodings_escaping_enc(self):
        with StrEncodingsEscapingEnc.from_file('src/str_encodings.bin') as r:
            with self.assertRaises(LookupError) as cm:
                r.str1.v
            self.assertEqual(str(cm.exception), 'unknown encoding: {}'.format("ASCII\\\\x"))

            with self.assertRaises(LookupError) as cm:
                r.str2.v
            self.assertEqual(str(cm.exception), 'unknown encoding: {}'.format("UTF-8\\'x"))

            with self.assertRaises(LookupError) as cm:
                r.str3.v
            self.assertEqual(str(cm.exception), 'unknown encoding: {}'.format("SJIS\\\"x"))

            with self.assertRaises(LookupError) as cm:
                r.str4.v
            self.assertEqual(str(cm.exception), 'unknown encoding: {}'.format("IBM437\\nx"))
