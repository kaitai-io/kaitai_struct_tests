import unittest
import io
from kaitaistruct import KaitaiStream
from specwrite.common_spec import CommonSpec

from testwrite.eos_exception_u4 import EosExceptionU4

class TestEosExceptionU4(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestEosExceptionU4, self).__init__(*args, **kwargs)
        self.struct_class = EosExceptionU4
        self.src_filename = 'src/term_strz.bin'

    def test_read_write_roundtrip(self):
        self.skipTest("cannot use roundtrip because parsing is expected to fail")

    def test_eos_exception_u4(self):
        r = EosExceptionU4()

        data = EosExceptionU4.Data(None, r, r._root)
        data.prebuf = b"\x00\xFF\xFE"
        data.fail_int = 3000500200
        data._check()

        r.envelope = data
        r._check()

        with KaitaiStream(io.BytesIO(bytearray(12))) as out_io:
            with self.assertRaisesRegex(EOFError, u"^requested to write 4 bytes, but only 3 bytes left in the stream$"):
                r._write(out_io)
