# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.nav_parent import NavParent

class TestNavParent(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestNavParent, self).__init__(*args, **kwargs)
        self.struct_class = NavParent
        self.src_filename = 'src/nav.bin'
