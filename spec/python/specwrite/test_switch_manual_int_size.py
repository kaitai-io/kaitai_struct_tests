import unittest
from kaitaistruct import ConsistencyError
from specwrite.common_spec import CommonSpec

from testwrite.switch_manual_int_size import SwitchManualIntSize

class TestSwitchManualIntSize(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestSwitchManualIntSize, self).__init__(*args, **kwargs)
        self.struct_class = SwitchManualIntSize
        self.src_filename = 'src/switch_tlv.bin'

    def test_check_switch_bytes_size_mismatch(self):
        chunk = SwitchManualIntSize.Chunk()
        chunk.code = 0x33
        chunk.size = 3
        # should cause the consistency check to fail because it's 2 bytes, not 3
        chunk.body = b"\x10\x20"

        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: body, expected: 3, actual: 2$"):
            chunk._check()
