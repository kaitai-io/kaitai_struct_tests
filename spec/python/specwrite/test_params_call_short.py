# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.params_call_short import ParamsCallShort

class TestParamsCallShort(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestParamsCallShort, self).__init__(*args, **kwargs)
        self.struct_class = ParamsCallShort
        self.src_filename = 'src/term_strz.bin'
