# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.params_enum import ParamsEnum

class TestParamsEnum(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestParamsEnum, self).__init__(*args, **kwargs)
        self.struct_class = ParamsEnum
        self.src_filename = 'src/enum_0.bin'
