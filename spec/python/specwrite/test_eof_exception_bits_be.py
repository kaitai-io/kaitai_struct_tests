from decorators import stream_param_tests, write_stream_param
from specwrite.common_spec import CommonSpec

from testwrite.eof_exception_bits_be import EofExceptionBitsBe

@stream_param_tests
class TestEofExceptionBitsBe(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestEofExceptionBitsBe, self).__init__(*args, **kwargs)
        self.struct_class = EofExceptionBitsBe
        self.src_filename = 'src/nav_parent_switch.bin'
        self.skip_roundtrip_msg_reason = "cannot use roundtrip because parsing is expected to fail"

    @write_stream_param
    def test_eof_exception_bits_be(self, stream_builder):
        r = EofExceptionBitsBe()
        r.pre_bits = 0b0000000  # 0b0000_000
        r.fail_bits = 0b101000010111111110  # 0b1_01000010_11111111_0
        r._check()

        with stream_builder(3) as obj:
            with obj.open() as ks_io:
                with self.assertRaisesRegexp(EOFError, u"^requested to write 3 bytes, but only 2 bytes left in the stream$"):
                    r._write__seq(ks_io)
                self.assertEqual(ks_io.pos(), 1)
