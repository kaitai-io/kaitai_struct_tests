from decorators import stream_param_tests, write_stream_param
from specwrite.common_spec import CommonSpec

from testwrite.eof_exception_u4 import EofExceptionU4

@stream_param_tests
class TestEofExceptionU4(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestEofExceptionU4, self).__init__(*args, **kwargs)
        self.struct_class = EofExceptionU4
        self.src_filename = 'src/term_strz.bin'
        self.skip_roundtrip_msg_reason = "cannot use roundtrip because parsing is expected to fail"

    @write_stream_param
    def test_eof_exception_u4(self, stream_builder):
        r = EofExceptionU4()
        r.prebuf = b"\x78\x79\x7A\x7B\x7C\x7D\x7E\x7F\x80"
        r.fail_int = 3000500200
        r._check()

        with stream_builder(12) as obj:
            with obj.open() as ks_io:
                with self.assertRaisesRegexp(EOFError, u"^requested to write 4 bytes, but only 3 bytes left in the stream$"):
                    r._write(ks_io)
