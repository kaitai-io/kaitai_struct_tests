import unittest

from switch_manual_enum import SwitchManualEnum

class TestSwitchManualEnum(unittest.TestCase):
    def test_switch_manual_enum(self):
        r = SwitchManualEnum.from_file("src/switch_opcodes.bin")

        self.assertEquals(r.opcodes.size, 4)

        self.assertEquals(r.opcodes[0].code, SwitchManualEnum.Opcode.Code.strval)
        self.assertEquals(r.opcodes[0].body.value, "foobar")

        self.assertEquals(r.opcodes[1].code, SwitchManualEnum.Opcode.Code.intval)
        self.assertEquals(r.opcodes[1].body.value, 0x42)

        self.assertEquals(r.opcodes[2].code, SwitchManualEnum.Opcode.Code.intval)
        self.assertEquals(r.opcodes[2].body.value, 0x37)

        self.assertEquals(r.opcodes[3].code, SwitchManualEnum.Opcode.Code.strval)
        self.assertEquals(r.opcodes[3].body.value, "")
