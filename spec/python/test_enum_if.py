import unittest

from enum_if import EnumIf

class TestEnumIf(unittest.TestCase):
    def test_enum_if(self):
        with EnumIf.from_file("src/if_struct.bin") as r:

            self.assertEqual(r.op1.opcode, EnumIf.Opcodes.a_string)
            self.assertEqual(r.op1.arg_str.str, "foo")

            self.assertEqual(r.op2.opcode, EnumIf.Opcodes.a_tuple)
            self.assertEqual(r.op2.arg_tuple.num1, 0x42)
            self.assertEqual(r.op2.arg_tuple.num2, 0x43)

            self.assertEqual(r.op3.opcode, EnumIf.Opcodes.a_string)
            self.assertEqual(r.op3.arg_str.str, "bar")
