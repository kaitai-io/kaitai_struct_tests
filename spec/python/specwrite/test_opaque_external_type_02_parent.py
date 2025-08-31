import unittest
from specwrite.common_spec import CommonSpec

from testwrite.opaque_external_type_02_parent import OpaqueExternalType02Parent

class TestOpaqueExternalType02Parent(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestOpaqueExternalType02Parent, self).__init__(*args, **kwargs)
        self.struct_class = OpaqueExternalType02Parent
        self.src_filename = 'src/term_strz.bin'
