# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from specwrite.common_spec import CommonSpec

from testwrite.debug_switch_user import DebugSwitchUser

class TestDebugSwitchUser(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestDebugSwitchUser, self).__init__(*args, **kwargs)
        self.struct_class = DebugSwitchUser
        self.src_filename = 'src/nav_parent_switch.bin'
