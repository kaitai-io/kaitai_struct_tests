import unittest
import io
from kaitaistruct import KaitaiStream
from specwrite.common_spec import CommonSpec

from testwrite.expr_io_eof_bits import ExprIoEofBits

class TestExprIoEofBits(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestExprIoEofBits, self).__init__(*args, **kwargs)
        self.struct_class = ExprIoEofBits
        self.src_filename = 'src/nav_parent_switch.bin'

    def test_read_write_roundtrip(self):
        self.skipTest("cannot use roundtrip because parsing is expected to fail")

    def test_expr_io_eof_bits(self):
        with KaitaiStream(io.open(self.src_filename, 'rb')) as orig_io:
            orig_io_size = orig_io.size()

        r = ExprIoEofBits()
        r.foo = 5167
        r.bar = 15
        r.assert_io_eof_before_baz = b""
        r.baz = 6
        r.assert_io_eof_after_baz = b"\x00" * 8  # doesn't matter
        r._check()

        new_io = KaitaiStream(io.BytesIO(bytearray(orig_io_size)))
        r._write__seq(new_io)

        with self.assertRaisesRegexp(EOFError, u"^requested to write 1 bytes, but only 0 bytes left in the stream$"):
            r.close()
