import unittest

from switch_manual_int import SwitchManualInt

class TestSwitchManualInt(unittest.TestCase):
    def test_switch_manual_int(self):
        r = SwitchManualInt.from_file("src/switch_opcodes.bin")

        self.assertEquals(r.opcodes.size, 4)

        self.assertEquals(r.opcodes[0].code, 83)
        self.assertEquals(r.opcodes[0].body.value, "foobar")

        self.assertEquals(r.opcodes[1].code, 73)
        self.assertEquals(r.opcodes[1].body.value, 0x42)

        self.assertEquals(r.opcodes[2].code, 73)
        self.assertEquals(r.opcodes[2].body.value, 0x37)

        self.assertEquals(r.opcodes[3].code, 83)
        self.assertEquals(r.opcodes[3].body.value, "")
