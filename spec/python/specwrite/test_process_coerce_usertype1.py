# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.process_coerce_usertype1 import ProcessCoerceUsertype1

class TestProcessCoerceUsertype1(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestProcessCoerceUsertype1, self).__init__(*args, **kwargs)
        self.struct_class = ProcessCoerceUsertype1
        self.src_filename = 'src/process_coerce_bytes.bin'
