# -*- coding: utf-8 -*-
import unittest
from kaitaistruct import ConsistencyError
from specwrite.common_spec import CommonSpec

from testwrite.str_encodings import StrEncodings

class TestStrEncodings(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestStrEncodings, self).__init__(*args, **kwargs)
        self.struct_class = StrEncodings
        self.src_filename = 'src/str_encodings.bin'

    def test_check_null(self):
        r = StrEncodings()

        r.str1 = "woo"
        r.len_of_1 = 3

        r.len_of_2 = 15

        with self.assertRaisesRegex(AttributeError, u"\\bstr2\\b"):
            r._check()

    def test_check_mismatch(self):
        r = StrEncodings()

        r.str1 = u"Some ASCII"
        r.str2 = u"こんにちは"
        r.str3 = u"こんにちは"
        r.str4 = u"░▒▓"

        # To be auto-derived
        r.len_of_1 = 10
        r.len_of_2 = 12  # should be 15
        r.len_of_3 = 10
        r.len_of_4 = 3

        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str2,"):
            r._check()
