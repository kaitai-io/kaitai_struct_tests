# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.combine_enum import CombineEnum

class TestCombineEnum(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestCombineEnum, self).__init__(*args, **kwargs)
        self.struct_class = CombineEnum
        self.src_filename = 'src/enum_0.bin'
