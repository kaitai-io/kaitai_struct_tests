# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.type_ternary_opaque import TypeTernaryOpaque

class TestTypeTernaryOpaque(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestTypeTernaryOpaque, self).__init__(*args, **kwargs)
        self.struct_class = TypeTernaryOpaque
        self.src_filename = 'src/term_strz.bin'
