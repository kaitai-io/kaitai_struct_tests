import unittest
import io
from kaitaistruct import KaitaiStream
from specwrite.common_spec import CommonSpec

from testwrite.eof_exception_bits_le import EofExceptionBitsLe

class TestEofExceptionBitsLe(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestEofExceptionBitsLe, self).__init__(*args, **kwargs)
        self.struct_class = EofExceptionBitsLe
        self.src_filename = 'src/nav_parent_switch.bin'

    def test_read_write_roundtrip(self):
        self.skipTest("cannot use roundtrip because parsing is expected to fail")

    def test_eof_exception_bits_le(self):
        r = EofExceptionBitsLe()
        r.pre_bits = 0b0000001  # 0b000_0001
        r.fail_bits = 0b011111111010000100  # 0b0_11111111_01000010_0
        r._check()

        with KaitaiStream(io.BytesIO(bytearray(3))) as out_io:
            with self.assertRaisesRegex(EOFError, u"^requested to write 3 bytes, but only 2 bytes left in the stream$"):
                r._write__seq(out_io)
            self.assertEqual(out_io.pos(), 1)
