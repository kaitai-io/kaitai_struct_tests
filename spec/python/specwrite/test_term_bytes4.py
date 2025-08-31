import io
import unittest
from kaitaistruct import ConsistencyError, KaitaiStream
from specwrite.common_spec import CommonSpec

from testwrite.term_bytes4 import TermBytes4

class TestTermBytes4(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestTermBytes4, self).__init__(*args, **kwargs)
        self.struct_class = TermBytes4
        self.src_filename = 'src/term_strz.bin'

    def test_check_is_eof_after_include_term_missing(self):
        r = TermBytes4()

        s1 = TermBytes4.S1Type(None, r, r._root)
        s1.value = b"foo"
        s1._check()
        r.s1 = s1

        r.skip_term1 = 0x7c

        s2 = TermBytes4.S2Type(None, r, r._root)
        s2.value = b"bar"
        s2._check()
        r.s2 = s2

        r.skip_term2 = 0x7c

        s3 = TermBytes4.S3Type(None, r, r._root)
        # this field with `include: true` and `eos-error: false` does not end with the terminator,
        # but there is 1 byte left in the stream => consistency error
        s3.value = b"ba"
        s3._check()
        r.s3 = s3

        r._check()

        len_io = (
            3 +  # s1
            1 +  # skip_term1
            3 +  # s2
            1 +  # skip_term2
            3  # s3
        )
        out_io = KaitaiStream(io.BytesIO(bytearray(len_io)))

        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: value, expected: 0, actual: 1$"):
            r._write(out_io)  # should throw a ConsistencyError
