import io
import unittest
from kaitaistruct import ConsistencyError, KaitaiStream
from specwrite.common_spec import CommonSpec

from testwrite.str_eos import StrEos

class TestStrEos(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestStrEos, self).__init__(*args, **kwargs)
        self.struct_class = StrEos
        self.src_filename = 'src/term_strz.bin'

    def test_check_longer_io(self):
        r = StrEos()

        r.str = "Hello"

        r._check()

        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: str, expected: 0, actual: 2$"):
            r._write(KaitaiStream(io.BytesIO(bytearray(5 + 2))))
