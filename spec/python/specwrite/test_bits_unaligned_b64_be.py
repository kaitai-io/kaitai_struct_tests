# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.bits_unaligned_b64_be import BitsUnalignedB64Be

class TestBitsUnalignedB64Be(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestBitsUnalignedB64Be, self).__init__(*args, **kwargs)
        self.struct_class = BitsUnalignedB64Be
        self.src_filename = 'src/process_xor_4.bin'
