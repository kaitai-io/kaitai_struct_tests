# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.nested_types import NestedTypes

class TestNestedTypes(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestNestedTypes, self).__init__(*args, **kwargs)
        self.struct_class = NestedTypes
        self.src_filename = 'src/fixed_struct.bin'
