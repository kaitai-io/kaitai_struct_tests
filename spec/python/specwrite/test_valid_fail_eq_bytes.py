# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.valid_fail_eq_bytes import ValidFailEqBytes

class TestValidFailEqBytes(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestValidFailEqBytes, self).__init__(*args, **kwargs)
        self.struct_class = ValidFailEqBytes
        self.src_filename = 'src/fixed_struct.bin'
        self.skip_roundtrip_msg_reason = "cannot use roundtrip because parsing is expected to fail"
