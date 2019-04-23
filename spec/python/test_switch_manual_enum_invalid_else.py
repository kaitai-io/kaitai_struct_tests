import unittest

from switch_manual_enum_invalid_else import SwitchManualEnumInvalidElse

class TestSwitchManualEnumInvalidElse(unittest.TestCase):
    def test_switch_manual_enum_invalid_else(self):
        r = SwitchManualEnumInvalidElse.from_file("src/enum_negative.bin")

        self.assertEqual(len(r.opcodes), 2)

        self.assertEqual(r.opcodes[0].code, 255)
        self.assertEqual(r.opcodes[0].body.value, 123)

        self.assertEqual(r.opcodes[1].code, 1)
        self.assertEqual(r.opcodes[1].body.value, 123)
