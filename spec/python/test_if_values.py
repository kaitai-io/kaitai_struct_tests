import unittest

from if_values import IfValues

class TestIfValues(unittest.TestCase):
    def test_if_values(self):
        r = IfValues.from_file("src/fixed_struct.bin")

        self.assertEqual(r.codes[0].opcode, 80)
        self.assertEqual(r.codes[0].half_opcode, 40)
        self.assertEqual(r.codes[1].opcode, 65)
        self.assertIsNone(r.codes[1].half_opcode)
        self.assertEqual(r.codes[2].opcode, 67)
        self.assertIsNone(r.codes[2].half_opcode)
