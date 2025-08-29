import unittest
from kaitaistruct import ConsistencyError
from specwrite.common_spec import CommonSpec

from testwrite.bytes_pad_term_equal import BytesPadTermEqual

class TestBytesPadTermEqual(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestBytesPadTermEqual, self).__init__(*args, **kwargs)
        self.struct_class = BytesPadTermEqual
        self.src_filename = 'src/str_pad_term.bin'

    def test_check_good_max_lengths(self):
        r = BytesPadTermEqual()
        r.s1 = b"12345678901234567890"
        r.s2 = b"12345678901234567890"
        r.s3 = b"12345678901234567890"
        r.s4 = b"12345678901234567890"
        r._check()

    def test_check_good_min_lengths(self):
        r = BytesPadTermEqual()
        r.s1 = b""
        r.s2 = b""
        r.s3 = b""
        r.s4 = b"."
        r._check()

    def test_check_longer_s1(self):
        r = BytesPadTermEqual()
        r.s1 = b"123456789012345678901"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: s1,"):
            r._check()

    def test_check_bad_has_terminator1_s1(self):
        r = BytesPadTermEqual()
        r.s1 = b"123456789012@4567890"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: s1,"):
            r._check()

    def test_check_bad_has_terminator2_s1(self):
        r = BytesPadTermEqual()
        r.s1 = b"1234567890123456789@"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: s1,"):
            r._check()

    def test_check_longer_s2(self):
        r = BytesPadTermEqual()
        r.s1 = b"12345678901234567890"
        r.s2 = b"123456789012345678901"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: s2,"):
            r._check()

    def test_check_good_terminator1_s2(self):
        r = BytesPadTermEqual()
        r.s1 = b"12345678901234567890"
        r.s2 = b"123456789012345678@"
        r.s3 = b"12345678901234567890"
        r.s4 = b"12345678901234567890"
        r._check()

    def test_check_early_terminator1_s2(self):
        r = BytesPadTermEqual()
        r.s1 = b"12345678901234567890"
        r.s2 = b"1234567890123456@8@"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: s2,"):
            r._check()

    def test_check_good_last_byte_s2(self):
        r = BytesPadTermEqual()
        r.s1 = b"12345678901234567890"
        r.s2 = b"+++++" b"+++++" b"+++++" b"+++9"
        r.s3 = b"12345678901234567890"
        r.s4 = b"12345678901234567890"
        r._check()

    def test_check_bad_last_byte_s2(self):
        r = BytesPadTermEqual()
        r.s1 = b"12345678901234567890"
        r.s2 = b"123456789012345678+"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: s2,"):
            r._check()

    def test_check_longer_s3(self):
        r = BytesPadTermEqual()
        r.s1 = b"12345678901234567890"
        r.s2 = b"12345678901234567890"
        r.s3 = b"123456789012345678901"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: s3,"):
            r._check()

    def test_check_bad_has_terminator1_s3(self):
        r = BytesPadTermEqual()
        r.s1 = b"12345678901234567890"
        r.s2 = b"12345678901234567890"
        r.s3 = b"1234567890123456789+"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: s3,"):
            r._check()

    def test_check_bad_has_terminator2_s3(self):
        r = BytesPadTermEqual()
        r.s1 = b"12345678901234567890"
        r.s2 = b"12345678901234567890"
        r.s3 = b"12345678901234567+9"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: s3,"):
            r._check()

    def test_check_longer_s4(self):
        r = BytesPadTermEqual()
        r.s1 = b"12345678901234567890"
        r.s2 = b"12345678901234567890"
        r.s3 = b"12345678901234567890"
        r.s4 = b"123456789012345678901"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: s4,"):
            r._check()

    def test_check_empty_s4(self):
        r = BytesPadTermEqual()
        r.s1 = b"12345678901234567890"
        r.s2 = b"12345678901234567890"
        r.s3 = b"12345678901234567890"
        r.s4 = b""
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: s4,"):
            r._check()

    def test_check_bad_no_terminator_s4(self):
        r = BytesPadTermEqual()
        r.s1 = b"12345678901234567890"
        r.s2 = b"12345678901234567890"
        r.s3 = b"12345678901234567890"
        r.s4 = b"1234567890123456789"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: s4,"):
            r._check()

    def test_check_good_terminator1_s4(self):
        r = BytesPadTermEqual()
        r.s1 = b"12345678901234567890"
        r.s2 = b"12345678901234567890"
        r.s3 = b"12345678901234567890"
        r.s4 = b"123456789012345678."
        r._check()

    def test_check_early_terminator1_s4(self):
        r = BytesPadTermEqual()
        r.s1 = b"12345678901234567890"
        r.s2 = b"12345678901234567890"
        r.s3 = b"12345678901234567890"
        r.s4 = b".23456789012345678."
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: s4,"):
            r._check()

    def test_check_good_terminator2_s4(self):
        r = BytesPadTermEqual()
        r.s1 = b"12345678901234567890"
        r.s2 = b"12345678901234567890"
        r.s3 = b"12345678901234567890"
        r.s4 = b"1234567890123456789."
        r._check()

    def test_check_early_terminator2_s4(self):
        r = BytesPadTermEqual()
        r.s1 = b"12345678901234567890"
        r.s2 = b"12345678901234567890"
        r.s3 = b"12345678901234567890"
        r.s4 = b"123456789012345678.."
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: s4,"):
            r._check()
