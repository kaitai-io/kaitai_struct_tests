import unittest

from switch_manual_str_else import SwitchManualStrElse

class TestSwitchManualStrElse(unittest.TestCase):
    def test_switch_manual_str_else(self):
        with SwitchManualStrElse.from_file("src/switch_opcodes2.bin") as r:

            self.assertEqual(len(r.opcodes), 4)

            self.assertEqual(r.opcodes[0].code, "S")
            self.assertEqual(r.opcodes[0].body.value, "foo")

            self.assertEqual(r.opcodes[1].code, "X")
            self.assertEqual(r.opcodes[1].body.filler, 0x42)

            self.assertEqual(r.opcodes[2].code, "Y")
            self.assertEqual(r.opcodes[2].body.filler, 0xcafe)

            self.assertEqual(r.opcodes[3].code, "I")
            self.assertEqual(r.opcodes[3].body.value, 7)
