# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.enum_to_i_invalid import EnumToIInvalid

class TestEnumToIInvalid(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestEnumToIInvalid, self).__init__(*args, **kwargs)
        self.struct_class = EnumToIInvalid
        self.src_filename = 'src/term_strz.bin'
