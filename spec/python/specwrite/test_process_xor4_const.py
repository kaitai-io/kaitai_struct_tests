# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.process_xor4_const import ProcessXor4Const

class TestProcessXor4Const(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestProcessXor4Const, self).__init__(*args, **kwargs)
        self.struct_class = ProcessXor4Const
        self.src_filename = 'src/process_xor_4.bin'
