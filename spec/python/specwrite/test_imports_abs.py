# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.imports_abs import ImportsAbs

class TestImportsAbs(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestImportsAbs, self).__init__(*args, **kwargs)
        self.struct_class = ImportsAbs
        self.src_filename = 'src/fixed_struct.bin'
