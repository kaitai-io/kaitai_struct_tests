import unittest

from switch_cast import SwitchCast

class TestSwitchCast(unittest.TestCase):
    def test_switch_cast(self):
        r = SwitchCast.from_file("src/switch_opcodes.bin")

        self.assertEqual(r.first_obj.value, "foobar")
        self.assertEqual(r.second_val, 0x42)
        # unable to test "err_cast" here
