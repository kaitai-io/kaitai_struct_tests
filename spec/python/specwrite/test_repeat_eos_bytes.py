import io
import unittest
from kaitaistruct import ConsistencyError, KaitaiStream
from specwrite.common_spec import CommonSpec

from testwrite.repeat_eos_bytes import RepeatEosBytes

class TestRepeatEosBytes(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestRepeatEosBytes, self).__init__(*args, **kwargs)
        self.struct_class = RepeatEosBytes
        self.src_filename = 'src/repeat_until_process.bin'

    def test_check_longer_io(self):
        r = RepeatEosBytes()

        r.records = [
            b"\xE8\xBA\xAA\xAA\xAA",
            b"\xFA\x9E\xB8\xAA\xAA",
            b"\xAA\x55\x55\x55\x55",
        ]

        r._check()

        ks_io = KaitaiStream(io.BytesIO(bytearray(15 + 3)))
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: records, expected: 0, actual: 3$"):
            r._write(ks_io)
