import unittest

from switch_manual_str import SwitchManualStr

class TestSwitchManualStr(unittest.TestCase):
    def test_switch_manual_str(self):
        with SwitchManualStr.from_file("src/switch_opcodes.bin") as r:

            self.assertEqual(len(r.opcodes), 4)

            self.assertEqual(r.opcodes[0].code, "S")
            self.assertEqual(r.opcodes[0].body.value, "foobar")

            self.assertEqual(r.opcodes[1].code, "I")
            self.assertEqual(r.opcodes[1].body.value, 0x42)

            self.assertEqual(r.opcodes[2].code, "I")
            self.assertEqual(r.opcodes[2].body.value, 0x37)

            self.assertEqual(r.opcodes[3].code, "S")
            self.assertEqual(r.opcodes[3].body.value, "")
