import unittest
from testformats.switch_manual_enum_invalid import SwitchManualEnumInvalid
from testformats import switch_manual_enum_invalid

class TestSwitchManualEnumInvalid(unittest.TestCase):
    def test_switch_manual_enum_invalid(self):
        with SwitchManualEnumInvalid.from_file('src/enum_negative.bin') as r:
            self.assertEqual(len(r.opcodes), 2)
            self.assertEqual(r.opcodes[0].code, 255)
            self.assertFalse(hasattr(r.opcodes[0], 'body'))
            self.assertEqual(r.opcodes[1].code, switch_manual_enum_invalid.SwitchManualEnumInvalid.Opcode.CodeEnum.foo)
            self.assertFalse(hasattr(r.opcodes[1], 'body'))
