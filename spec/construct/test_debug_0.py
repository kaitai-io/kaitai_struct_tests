import unittest

from debug_0 import _schema

class TestDebug0(unittest.TestCase):
    def test_debug_0(self):
        r = _schema.parse_file('src/fixed_struct.bin')
        self.assertEqual(r.one, 80)
        self.assertEqual(len(r.array_of_ints), 3)
        self.assertEqual(r.array_of_ints[0], 65)
        self.assertEqual(r.array_of_ints[1], 67)
        self.assertEqual(r.array_of_ints[2], 75)
        self.assertEqual(r._unnamed2, 45)
