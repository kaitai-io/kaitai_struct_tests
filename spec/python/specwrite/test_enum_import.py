# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.enum_import import EnumImport

class TestEnumImport(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestEnumImport, self).__init__(*args, **kwargs)
        self.struct_class = EnumImport
        self.src_filename = 'src/enum_0.bin'
