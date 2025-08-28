import unittest
from testformats.debug_enum_name import DebugEnumName
from testformats import debug_enum_name

class TestDebugEnumName(unittest.TestCase):
    def test_debug_enum_name(self):
        with DebugEnumName.from_file('src/fixed_struct.bin') as r:
            # --debug implies --no-auto-read
            r._read()

            self.assertEqual(r.one, debug_enum_name.DebugEnumName.TestEnum1.enum_value_80)
            self.assertEqual(r.array_of_ints[0], debug_enum_name.DebugEnumName.TestEnum2.enum_value_65)
            self.assertEqual(r.test_type.field1, debug_enum_name.DebugEnumName.TestSubtype.InnerEnum1.enum_value_67)
            self.assertEqual(r.test_type.instance_field, debug_enum_name.DebugEnumName.TestSubtype.InnerEnum2.enum_value_11)
