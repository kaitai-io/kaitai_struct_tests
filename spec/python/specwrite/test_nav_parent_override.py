# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.nav_parent_override import NavParentOverride

class TestNavParentOverride(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestNavParentOverride, self).__init__(*args, **kwargs)
        self.struct_class = NavParentOverride
        self.src_filename = 'src/nav_parent_codes.bin'
