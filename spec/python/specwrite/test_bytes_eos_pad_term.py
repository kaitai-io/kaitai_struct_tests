import io
import unittest
from kaitaistruct import ConsistencyError, KaitaiStream
from specwrite.common_spec import CommonSpec

from testwrite.bytes_eos_pad_term import BytesEosPadTerm

class TestBytesEosPadTerm(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestBytesEosPadTerm, self).__init__(*args, **kwargs)
        self.struct_class = BytesEosPadTerm
        self.src_filename = 'src/str_pad_term.bin'

    def test_check_good_max_lengths(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"12345678901234567890")
        set_str_term(r, b"12345678901234567890")
        set_str_term_and_pad(r, b"12345678901234567890")
        set_str_term_include(r, b"12345678901234567890")
        check(r)

    def test_check_good_min_lengths(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"")
        set_str_term(r, b"")
        set_str_term_and_pad(r, b"")
        set_str_term_include(r, b"@")
        check(r)

    def test_check_longer_str_pad(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"123456789012345678901")
        with self.assertRaisesRegex(EOFError, u"^requested to write \\d+ bytes, but only \\d+ bytes left in the stream$"):
            check(r)

    def test_check_good_last_byte_str_pad(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"@@@@@" b"@@@@@" b"@@@@@" b"@@@@?")
        set_str_term(r, b"12345678901234567890")
        set_str_term_and_pad(r, b"12345678901234567890")
        set_str_term_include(r, b"12345678901234567890")
        check(r)

    def test_check_bad_last_byte_str_pad(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"123456789012345678@")
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_pad,"):
            check(r)

    def test_check_longer_str_term(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"12345678901234567890")
        set_str_term(r, b"123456789012345678901")
        with self.assertRaisesRegex(EOFError, u"^requested to write \\d+ bytes, but only \\d+ bytes left in the stream$"):
            check(r)

    def test_check_bad_has_terminator1_str_term(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"12345678901234567890")
        set_str_term(r, b"123456789012@4567890")
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term,"):
            check(r)

    def test_check_bad_has_terminator2_str_term(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"12345678901234567890")
        set_str_term(r, b"1234567890123456789@")
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term,"):
            check(r)

    def test_check_longer_str_term_and_pad(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"12345678901234567890")
        set_str_term(r, b"12345678901234567890")
        set_str_term_and_pad(r, b"123456789012345678901")
        with self.assertRaisesRegex(EOFError, u"^requested to write \\d+ bytes, but only \\d+ bytes left in the stream$"):
            check(r)

    def test_check_bad_has_terminator_str_term_and_pad(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"12345678901234567890")
        set_str_term(r, b"12345678901234567890")
        set_str_term_and_pad(r, b"@2345678901234567890")
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term_and_pad,"):
            check(r)

    def test_check_good_last_byte1_str_term_and_pad(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"12345678901234567890")
        set_str_term(r, b"12345678901234567890")
        set_str_term_and_pad(r, b"+++++" b"+++++" b"+++++" b"++++")
        set_str_term_include(r, b"12345678901234567890")
        check(r)

    def test_check_good_last_byte2_str_term_and_pad(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"12345678901234567890")
        set_str_term(r, b"12345678901234567890")
        set_str_term_and_pad(r, b"+++++" b"+++++" b"+++++" b"++++0")
        set_str_term_include(r, b"12345678901234567890")
        check(r)

    def test_check_bad_last_byte_str_term_and_pad(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"12345678901234567890")
        set_str_term(r, b"12345678901234567890")
        set_str_term_and_pad(r, b"1234567890123456789+")
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term_and_pad,"):
            check(r)

    def test_check_longer_str_term_include(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"12345678901234567890")
        set_str_term(r, b"12345678901234567890")
        set_str_term_and_pad(r, b"12345678901234567890")
        set_str_term_include(r, b"123456789012345678901")
        with self.assertRaisesRegex(EOFError, u"^requested to write \\d+ bytes, but only \\d+ bytes left in the stream$"):
            check(r)

    def test_check_empty_str_term_include(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"12345678901234567890")
        set_str_term(r, b"12345678901234567890")
        set_str_term_and_pad(r, b"12345678901234567890")
        set_str_term_include(r, b"")
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term_include,"):
            check(r)

    def test_check_bad_no_terminator_str_term_include(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"12345678901234567890")
        set_str_term(r, b"12345678901234567890")
        set_str_term_and_pad(r, b"12345678901234567890")
        set_str_term_include(r, b"123")
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term_include,"):
            check(r)

    def test_check_good_terminator1_str_term_include(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"12345678901234567890")
        set_str_term(r, b"12345678901234567890")
        set_str_term_and_pad(r, b"12345678901234567890")
        set_str_term_include(r, b"12@")
        check(r)

    def test_check_early_terminator1_str_term_include(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"12345678901234567890")
        set_str_term(r, b"12345678901234567890")
        set_str_term_and_pad(r, b"12345678901234567890")
        set_str_term_include(r, b"1@@")
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term_include,"):
            check(r)

    def test_check_good_terminator2_str_term_include(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"12345678901234567890")
        set_str_term(r, b"12345678901234567890")
        set_str_term_and_pad(r, b"12345678901234567890")
        set_str_term_include(r, b"1234567890123456789@")
        check(r)

    def test_check_early_terminator2_str_term_include(self):
        r = BytesEosPadTerm()
        set_str_pad(r, b"12345678901234567890")
        set_str_term(r, b"12345678901234567890")
        set_str_term_and_pad(r, b"12345678901234567890")
        set_str_term_include(r, b"123456789012345678@@")
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str_term_include,"):
            check(r)


def set_str_pad(r, value):
    s = BytesEosPadTerm.StrPadType(None, r, r._root)
    s.value = value
    s._check()

    r.str_pad = s


def set_str_term(r, value):
    s = BytesEosPadTerm.StrTermType(None, r, r._root)
    s.value = value
    s._check()

    r.str_term = s


def set_str_term_and_pad(r, value):
    s = BytesEosPadTerm.StrTermAndPadType(None, r, r._root)
    s.value = value
    s._check()

    r.str_term_and_pad = s


def set_str_term_include(r, value):
    s = BytesEosPadTerm.StrTermIncludeType(None, r, r._root)
    s.value = value
    s._check()

    r.str_term_include = s


def check(r):
    buf = b"12345678901234567890"
    if not hasattr(r, 'str_pad'):
        set_str_pad(r, buf)
    if not hasattr(r, 'str_term'):
        set_str_term(r, buf)
    if not hasattr(r, 'str_term_and_pad'):
        set_str_term_and_pad(r, buf)
    if not hasattr(r, 'str_term_include'):
        set_str_term_include(r, buf)

    r._check()
    ks_io = KaitaiStream(io.BytesIO(bytearray(80)))
    r._write(ks_io)
