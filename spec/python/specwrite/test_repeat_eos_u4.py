# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.repeat_eos_u4 import RepeatEosU4

class TestRepeatEosU4(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestRepeatEosU4, self).__init__(*args, **kwargs)
        self.struct_class = RepeatEosU4
        self.src_filename = 'src/repeat_eos_struct.bin'
