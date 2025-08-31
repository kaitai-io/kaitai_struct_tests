import unittest
import io
from kaitaistruct import KaitaiStream
from specwrite.common_spec import CommonSpec

from testwrite.eof_exception_bytes import EofExceptionBytes

class TestEofExceptionBytes(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestEofExceptionBytes, self).__init__(*args, **kwargs)
        self.struct_class = EofExceptionBytes
        self.src_filename = 'src/term_strz.bin'

    def test_read_write_roundtrip(self):
        self.skipTest("cannot use roundtrip because parsing is expected to fail")

    def test_eof_exception_bytes(self):
        r = EofExceptionBytes()
        r.buf = b"\x78\x79\x7A\x7B\x7C\x7D\x7E\x7F\xFF\xFE\xFD\xFC\xFB"
        r._check()

        with KaitaiStream(io.BytesIO(bytearray(12))) as out_io:
            with self.assertRaisesRegex(EOFError, u"^requested to write 13 bytes, but only 12 bytes left in the stream$"):
                r._write(out_io)
