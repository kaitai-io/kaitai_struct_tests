# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.params_call_extra_parens import ParamsCallExtraParens

class TestParamsCallExtraParens(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestParamsCallExtraParens, self).__init__(*args, **kwargs)
        self.struct_class = ParamsCallExtraParens
        self.src_filename = 'src/term_strz.bin'
