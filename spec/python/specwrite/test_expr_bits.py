# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.expr_bits import ExprBits

class TestExprBits(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestExprBits, self).__init__(*args, **kwargs)
        self.struct_class = ExprBits
        self.src_filename = 'src/switch_opcodes.bin'
