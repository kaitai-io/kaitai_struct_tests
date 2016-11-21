import unittest

from switch_integers import SwitchIntegers

class TestSwitchIntegers(unittest.TestCase):
    def test_switch_integers(self):
        r = SwitchIntegers.from_file("src/switch_integers.bin")

        self.assertEquals(len(r.opcodes), 4)

        self.assertEquals(r.opcodes[0].code, 1)
        self.assertEquals(r.opcodes[0].body, 7)

        self.assertEquals(r.opcodes[1].code, 2)
        self.assertEquals(r.opcodes[1].body, 0x4040)

        self.assertEquals(r.opcodes[2].code, 4)
        self.assertEquals(r.opcodes[2].body, 4919)

        self.assertEquals(r.opcodes[3].code, 8)
        self.assertEquals(r.opcodes[3].body, 4919)
