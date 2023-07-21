from decorators import stream_param_tests, write_stream_param
from specwrite.common_spec import CommonSpec

from testwrite.eos_exception_u4 import EosExceptionU4

@stream_param_tests
class TestEosExceptionU4(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestEosExceptionU4, self).__init__(*args, **kwargs)
        self.struct_class = EosExceptionU4
        self.src_filename = 'src/term_strz.bin'
        self.skip_roundtrip_msg_reason = "cannot use roundtrip because parsing is expected to fail"

    @write_stream_param
    def test_eos_exception_u4(self, stream_builder):
        r = EosExceptionU4()

        data = EosExceptionU4.Data(None, r, r._root)
        data.prebuf = b"\x00\xFF\xFE"
        data.fail_int = 3000500200
        data._check()

        r.envelope = data
        r._check()

        with stream_builder(12) as obj:
            with obj.open() as ks_io:
                with self.assertRaisesRegexp(EOFError, u"^requested to write 4 bytes, but only 3 bytes left in the stream$"):
                    r._write(ks_io)
