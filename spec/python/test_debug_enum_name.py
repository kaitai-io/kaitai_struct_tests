import unittest

from debug_enum_name import DebugEnumName

class TestDebugEnumName(unittest.TestCase):
    def test_debug_enum_name(self):
        r = DebugEnumName.from_file("src/fixed_struct.bin")

        # this test is meaningful only for languages that have --debug and do
        # not save enum type info
