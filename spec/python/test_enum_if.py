import unittest

from enum_if import EnumIf

class TestEnumIf(unittest.TestCase):
    def test_enum_if(self):
        r = EnumIf.from_file("src/if_struct.bin")

        self.assertEquals(r.op1.opcode, EnumIf.Opcodes.a_string)
        self.assertEquals(r.op1.arg_str.str, "foo")

        self.assertEquals(r.op2.opcode, EnumIf.Opcodes.a_tuple)
        self.assertEquals(r.op2.arg_tuple.num1, 0x42)
        self.assertEquals(r.op2.arg_tuple.num2, 0x43)

        self.assertEquals(r.op3.opcode, EnumIf.Opcodes.a_string)
        self.assertEquals(r.op3.arg_str.str, "bar")
