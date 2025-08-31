import unittest
from testformats.switch_cast import SwitchCast

class TestSwitchCast(unittest.TestCase):
    def test_switch_cast(self):
        with SwitchCast.from_file('src/switch_opcodes.bin') as r:
            self.assertEqual(r.first_obj.value, "foobar")
            self.assertEqual(r.second_val, 0x42)

            self.assertNotIsInstance(r.err_cast, SwitchCast.Strval)
            self.assertIsInstance(r.err_cast, SwitchCast.Intval)
