# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.repeat_until_term_struct import RepeatUntilTermStruct

class TestRepeatUntilTermStruct(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestRepeatUntilTermStruct, self).__init__(*args, **kwargs)
        self.struct_class = RepeatUntilTermStruct
        self.src_filename = 'src/repeat_until_process.bin'
