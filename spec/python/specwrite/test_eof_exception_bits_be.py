import unittest
import io
from kaitaistruct import KaitaiStream
from specwrite.common_spec import CommonSpec

from testwrite.eof_exception_bits_be import EofExceptionBitsBe

class TestEofExceptionBitsBe(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestEofExceptionBitsBe, self).__init__(*args, **kwargs)
        self.struct_class = EofExceptionBitsBe
        self.src_filename = 'src/nav_parent_switch.bin'

    def test_read_write_roundtrip(self):
        self.skipTest("cannot use roundtrip because parsing is expected to fail")

    def test_eof_exception_bits_be(self):
        r = EofExceptionBitsBe()
        r.pre_bits = 0b0000000  # 0b0000_000
        r.fail_bits = 0b101000010111111110  # 0b1_01000010_11111111_0
        r._check()

        with KaitaiStream(io.BytesIO(bytearray(3))) as out_io:
            with self.assertRaisesRegex(EOFError, u"^requested to write 3 bytes, but only 2 bytes left in the stream$"):
                r._write__seq(out_io)
            self.assertEqual(out_io.pos(), 1)
