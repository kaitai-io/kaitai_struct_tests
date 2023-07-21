from decorators import stream_param_tests, write_stream_param
from specwrite.common_spec import CommonSpec

from testwrite.eos_exception_bytes import EosExceptionBytes

@stream_param_tests
class TestEosExceptionBytes(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestEosExceptionBytes, self).__init__(*args, **kwargs)
        self.struct_class = EosExceptionBytes
        self.src_filename = 'src/term_strz.bin'
        self.skip_roundtrip_msg_reason = "cannot use roundtrip because parsing is expected to fail"

    @write_stream_param
    def test_eos_exception_bytes(self, stream_builder):
        r = EosExceptionBytes()

        data = EosExceptionBytes.Data(None, r, r._root)
        data.buf = b"\x00\xFF\xFE\xFD\xFC\xFB\xFA"
        data._check()

        r.envelope = data
        r._check()

        with stream_builder(12) as obj:
            with obj.open() as ks_io:
                with self.assertRaisesRegexp(EOFError, u"^requested to write 7 bytes, but only 6 bytes left in the stream$"):
                    r._write(ks_io)
