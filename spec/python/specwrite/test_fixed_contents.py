# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.fixed_contents import FixedContents

class TestFixedContents(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestFixedContents, self).__init__(*args, **kwargs)
        self.struct_class = FixedContents
        self.src_filename = 'src/fixed_struct.bin'
