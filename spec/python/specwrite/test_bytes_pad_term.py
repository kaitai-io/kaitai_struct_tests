import unittest
from kaitaistruct import ConsistencyError
from specwrite.common_spec import CommonSpec

from testwrite.bytes_pad_term import BytesPadTerm

class TestBytesPadTerm(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestBytesPadTerm, self).__init__(*args, **kwargs)
        self.struct_class = BytesPadTerm
        self.src_filename = 'src/str_pad_term.bin'

    def test_check_good_max_lengths(self):
        r = BytesPadTerm()
        r.str_pad = b"12345678901234567890"
        r.str_term = b"12345678901234567890"
        r.str_term_and_pad = b"12345678901234567890"
        r.str_term_include = b"12345678901234567890"
        r._check()

    def test_check_good_min_lengths(self):
        r = BytesPadTerm()
        r.str_pad = b""
        r.str_term = b""
        r.str_term_and_pad = b""
        r.str_term_include = b"@"
        r._check()

    def test_check_longer_str_pad(self):
        r = BytesPadTerm()
        r.str_pad = b"123456789012345678901"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_pad,"):
            r._check()

    def test_check_good_last_byte_str_pad(self):
        r = BytesPadTerm()
        r.str_pad = b"@@@@@" b"@@@@@" b"@@@@@" b"@@@@?"
        r.str_term = b"12345678901234567890"
        r.str_term_and_pad = b"12345678901234567890"
        r.str_term_include = b"12345678901234567890"
        r._check()

    def test_check_bad_last_byte_str_pad(self):
        r = BytesPadTerm()
        r.str_pad = b"123456789012345678@"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_pad,"):
            r._check()

    def test_check_longer_str_term(self):
        r = BytesPadTerm()
        r.str_pad = b"12345678901234567890"
        r.str_term = b"123456789012345678901"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term,"):
            r._check()

    def test_check_bad_has_terminator1_str_term(self):
        r = BytesPadTerm()
        r.str_pad = b"12345678901234567890"
        r.str_term = b"123456789012@4567890"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term,"):
            r._check()

    def test_check_bad_has_terminator2_str_term(self):
        r = BytesPadTerm()
        r.str_pad = b"12345678901234567890"
        r.str_term = b"1234567890123456789@"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term,"):
            r._check()

    def test_check_longer_str_term_and_pad(self):
        r = BytesPadTerm()
        r.str_pad = b"12345678901234567890"
        r.str_term = b"12345678901234567890"
        r.str_term_and_pad = b"123456789012345678901"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term_and_pad,"):
            r._check()

    def test_check_bad_has_terminator_str_term_and_pad(self):
        r = BytesPadTerm()
        r.str_pad = b"12345678901234567890"
        r.str_term = b"12345678901234567890"
        r.str_term_and_pad = b"@2345678901234567890"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term_and_pad,"):
            r._check()

    def test_check_good_last_byte1_str_term_and_pad(self):
        r = BytesPadTerm()
        r.str_pad = b"12345678901234567890"
        r.str_term = b"12345678901234567890"
        r.str_term_and_pad = b"+++++" b"+++++" b"+++++" b"++++"
        r.str_term_include = b"12345678901234567890"
        r._check()

    def test_check_good_last_byte2_str_term_and_pad(self):
        r = BytesPadTerm()
        r.str_pad = b"12345678901234567890"
        r.str_term = b"12345678901234567890"
        r.str_term_and_pad = b"+++++" b"+++++" b"+++++" b"++++0"
        r.str_term_include = b"12345678901234567890"
        r._check()

    def test_check_bad_last_byte_str_term_and_pad(self):
        r = BytesPadTerm()
        r.str_pad = b"12345678901234567890"
        r.str_term = b"12345678901234567890"
        r.str_term_and_pad = b"1234567890123456789+"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term_and_pad,"):
            r._check()

    def test_check_longer_str_term_include(self):
        r = BytesPadTerm()
        r.str_pad = b"12345678901234567890"
        r.str_term = b"12345678901234567890"
        r.str_term_and_pad = b"12345678901234567890"
        r.str_term_include = b"123456789012345678901"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term_include,"):
            r._check()

    def test_check_empty_str_term_include(self):
        r = BytesPadTerm()
        r.str_pad = b"12345678901234567890"
        r.str_term = b"12345678901234567890"
        r.str_term_and_pad = b"12345678901234567890"
        r.str_term_include = b""
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term_include,"):
            r._check()

    def test_check_bad_no_terminator_str_term_include(self):
        r = BytesPadTerm()
        r.str_pad = b"12345678901234567890"
        r.str_term = b"12345678901234567890"
        r.str_term_and_pad = b"12345678901234567890"
        r.str_term_include = b"123"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term_include,"):
            r._check()

    def test_check_good_terminator1_str_term_include(self):
        r = BytesPadTerm()
        r.str_pad = b"12345678901234567890"
        r.str_term = b"12345678901234567890"
        r.str_term_and_pad = b"12345678901234567890"
        r.str_term_include = b"12@"
        r._check()

    def test_check_early_terminator1_str_term_include(self):
        r = BytesPadTerm()
        r.str_pad = b"12345678901234567890"
        r.str_term = b"12345678901234567890"
        r.str_term_and_pad = b"12345678901234567890"
        r.str_term_include = b"1@@"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term_include,"):
            r._check()

    def test_check_good_terminator2_str_term_include(self):
        r = BytesPadTerm()
        r.str_pad = b"12345678901234567890"
        r.str_term = b"12345678901234567890"
        r.str_term_and_pad = b"12345678901234567890"
        r.str_term_include = b"1234567890123456789@"
        r._check()

    def test_check_early_terminator2_str_term_include(self):
        r = BytesPadTerm()
        r.str_pad = b"12345678901234567890"
        r.str_term = b"12345678901234567890"
        r.str_term_and_pad = b"12345678901234567890"
        r.str_term_include = b"123456789012345678@@"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term_include,"):
            r._check()
