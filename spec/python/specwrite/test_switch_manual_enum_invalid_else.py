# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.switch_manual_enum_invalid_else import SwitchManualEnumInvalidElse

class TestSwitchManualEnumInvalidElse(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestSwitchManualEnumInvalidElse, self).__init__(*args, **kwargs)
        self.struct_class = SwitchManualEnumInvalidElse
        self.src_filename = 'src/enum_negative.bin'
