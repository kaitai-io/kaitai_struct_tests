# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.expr_array import ExprArray

class TestExprArray(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestExprArray, self).__init__(*args, **kwargs)
        self.struct_class = ExprArray
        self.src_filename = 'src/expr_array.bin'
