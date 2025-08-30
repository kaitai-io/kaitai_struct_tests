import unittest
import io
from kaitaistruct import KaitaiStream
from specwrite.common_spec import CommonSpec

from testwrite.eof_exception_u4 import EofExceptionU4

class TestEofExceptionU4(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestEofExceptionU4, self).__init__(*args, **kwargs)
        self.struct_class = EofExceptionU4
        self.src_filename = 'src/term_strz.bin'

    def test_read_write_roundtrip(self):
        self.skipTest("cannot use roundtrip because parsing is expected to fail")

    def test_eof_exception_u4(self):
        r = EofExceptionU4()
        r.prebuf = b"\x78\x79\x7A\x7B\x7C\x7D\x7E\x7F\x80"
        r.fail_int = 3000500200
        r._check()

        with KaitaiStream(io.BytesIO(bytearray(12))) as out_io:
            with self.assertRaisesRegex(EOFError, u"^requested to write 4 bytes, but only 3 bytes left in the stream$"):
                r._write(out_io)
