# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.repeat_n_bytes import RepeatNBytes

class TestRepeatNBytes(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestRepeatNBytes, self).__init__(*args, **kwargs)
        self.struct_class = RepeatNBytes
        self.src_filename = 'src/repeat_until_process.bin'
