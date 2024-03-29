# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from params_call import _schema

class TestParamsCall(unittest.TestCase):
    def test_params_call(self):
        r = _schema.parse_file('src/term_strz.bin')
        self.assertEqual(r.buf1.body, u"foo|b")
        self.assertEqual(r.buf2.body, u"ar|ba")
        self.assertEqual(r.buf2.trailer, 122)
