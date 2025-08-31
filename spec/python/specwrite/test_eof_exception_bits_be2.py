import unittest
import io
from kaitaistruct import KaitaiStream
from specwrite.common_spec import CommonSpec

from testwrite.eof_exception_bits_be2 import EofExceptionBitsBe2

class TestEofExceptionBitsBe2(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestEofExceptionBitsBe2, self).__init__(*args, **kwargs)
        self.struct_class = EofExceptionBitsBe2
        self.src_filename = 'src/nav_parent_switch.bin'

    def test_read_write_roundtrip(self):
        self.skipTest("cannot use roundtrip because parsing is expected to fail")

    def test_eof_exception_bits_be2(self):
        r = EofExceptionBitsBe2()
        r.pre_bits = 0x01
        r.fail_bits = 0b01000010111111110  # 0b01000010_11111111_0
        r._check()

        with KaitaiStream(io.BytesIO(bytearray(3))) as out_io:
            with self.assertRaisesRegex(EOFError, u"^requested to write 3 bytes, but only 2 bytes left in the stream$"):
                r._write__seq(out_io)
            self.assertEqual(out_io.pos(), 1)
