# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.bits_signed_shift_b32_le import BitsSignedShiftB32Le

class TestBitsSignedShiftB32Le(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestBitsSignedShiftB32Le, self).__init__(*args, **kwargs)
        self.struct_class = BitsSignedShiftB32Le
        self.src_filename = 'src/bits_signed_shift_b32_le.bin'
