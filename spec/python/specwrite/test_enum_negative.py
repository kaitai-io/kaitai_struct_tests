# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.enum_negative import EnumNegative

class TestEnumNegative(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestEnumNegative, self).__init__(*args, **kwargs)
        self.struct_class = EnumNegative
        self.src_filename = 'src/enum_negative.bin'
