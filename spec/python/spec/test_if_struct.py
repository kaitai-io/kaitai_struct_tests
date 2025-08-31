import unittest
from testformats.if_struct import IfStruct

class TestIfStruct(unittest.TestCase):
    def test_if_struct(self):
        with IfStruct.from_file('src/if_struct.bin') as r:
            self.assertEqual(r.op1.opcode, 83)
            self.assertFalse(hasattr(r.op1, 'arg_tuple'))
            self.assertEqual(r.op1.arg_str.str, u"foo")
            self.assertEqual(r.op2.opcode, 84)
            self.assertEqual(r.op2.arg_tuple.num1, 66)
            self.assertEqual(r.op2.arg_tuple.num2, 67)
            self.assertFalse(hasattr(r.op2, 'arg_str'))
            self.assertEqual(r.op3.opcode, 83)
            self.assertFalse(hasattr(r.op3, 'arg_tuple'))
            self.assertEqual(r.op3.arg_str.str, u"bar")
