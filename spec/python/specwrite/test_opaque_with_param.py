import unittest
from specwrite.common_spec import CommonSpec

from testwrite.opaque_with_param import OpaqueWithParam

class TestOpaqueWithParam(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestOpaqueWithParam, self).__init__(*args, **kwargs)
        self.struct_class = OpaqueWithParam
        self.src_filename = 'src/term_strz.bin'
