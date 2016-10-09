import unittest

from switch_manual_enum import SwitchManualEnum

class TestSwitchManualEnum(unittest.TestCase):
    def test_switch_manual_enum(self):
        r = SwitchManualEnum.from_file("src/switch_opcodes.bin")

        self.assertEqual(r.opcodes.size, 4)

        self.assertEqual(r.opcodes[0].code, SwitchManualEnum.Opcode.Code.strval)
        self.assertEqual(r.opcodes[0].body.value, "foobar")

        self.assertEqual(r.opcodes[1].code, SwitchManualEnum.Opcode.Code.intval)
        self.assertEqual(r.opcodes[1].body.value, 0x42)

        self.assertEqual(r.opcodes[2].code, SwitchManualEnum.Opcode.Code.intval)
        self.assertEqual(r.opcodes[2].body.value, 0x37)

        self.assertEqual(r.opcodes[3].code, SwitchManualEnum.Opcode.Code.strval)
        self.assertEqual(r.opcodes[3].body.value, "")
