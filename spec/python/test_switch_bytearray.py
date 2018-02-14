import unittest

from switch_bytearray import SwitchBytearray

class TestSwitchBytearray(unittest.TestCase):
    def test_switch_bytearray(self):
        r = SwitchBytearray.from_file("src/switch_opcodes.bin")

        self.assertEqual(len(r.opcodes), 4)

        self.assertEqual(r.opcodes[0].code, bytearray([83]))
        self.assertEqual(r.opcodes[0].body.value, "foobar")

        self.assertEqual(r.opcodes[1].code, bytearray([73]))
        self.assertEqual(r.opcodes[1].body.value, 0x42)

        self.assertEqual(r.opcodes[2].code, bytearray([73]))
        self.assertEqual(r.opcodes[2].body.value, 0x37)

        self.assertEqual(r.opcodes[3].code, bytearray([83]))
        self.assertEqual(r.opcodes[3].body.value, "")
