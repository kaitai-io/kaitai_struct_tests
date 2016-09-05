import unittest

from switch_manual_int_else import SwitchManualIntElse

class TestSwitchManualIntElse(unittest.TestCase):
    def test_switch_manual_int_else(self):
        r = SwitchManualIntElse.from_file("src/switch_opcodes2.bin")

        self.assertEquals(len(r.opcodes), 4)

        self.assertEquals(r.opcodes[0].code, 83)
        self.assertEquals(r.opcodes[0].body.value, "foo")

        self.assertEquals(r.opcodes[1].code, 88)
        self.assertEquals(r.opcodes[1].body.filler, 0x42)

        self.assertEquals(r.opcodes[2].code, 89)
        self.assertEquals(r.opcodes[2].body.filler, 0xcafe)

        self.assertEquals(r.opcodes[3].code, 73)
        self.assertEquals(r.opcodes[3].body.value, 7)
