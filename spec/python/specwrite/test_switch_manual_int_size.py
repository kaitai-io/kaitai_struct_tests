# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.switch_manual_int_size import SwitchManualIntSize

class TestSwitchManualIntSize(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestSwitchManualIntSize, self).__init__(*args, **kwargs)
        self.struct_class = SwitchManualIntSize
        self.src_filename = 'src/switch_tlv.bin'
