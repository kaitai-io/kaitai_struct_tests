from decorators import stream_param_tests, write_stream_param
from specwrite.common_spec import CommonSpec

from testwrite.eof_exception_bytes import EofExceptionBytes

@stream_param_tests
class TestEofExceptionBytes(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestEofExceptionBytes, self).__init__(*args, **kwargs)
        self.struct_class = EofExceptionBytes
        self.src_filename = 'src/term_strz.bin'
        self.skip_roundtrip_msg_reason = "cannot use roundtrip because parsing is expected to fail"

    @write_stream_param
    def test_eof_exception_bytes(self, stream_builder):
        r = EofExceptionBytes()
        r.buf = b"\x78\x79\x7A\x7B\x7C\x7D\x7E\x7F\xFF\xFE\xFD\xFC\xFB"
        r._check()

        with stream_builder(12) as obj:
            with obj.open() as ks_io:
                with self.assertRaisesRegexp(EOFError, u"^requested to write 13 bytes, but only 12 bytes left in the stream$"):
                    r._write(ks_io)
