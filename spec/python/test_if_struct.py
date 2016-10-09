import unittest

from if_struct import IfStruct

class TestIfStruct(unittest.TestCase):
    def test_if_struct(self):
        r = IfStruct.from_file("src/if_struct.bin")

        self.assertEqual(r.op1.opcode, 0x53)
        self.assertEqual(r.op1.arg_str.str, "foo")

        self.assertEqual(r.op2.opcode, 0x54)
        self.assertEqual(r.op2.arg_tuple.num1, 0x42)
        self.assertEqual(r.op2.arg_tuple.num2, 0x43)

        self.assertEqual(r.op3.opcode, 0x53)
        self.assertEqual(r.op3.arg_str.str, "bar")
