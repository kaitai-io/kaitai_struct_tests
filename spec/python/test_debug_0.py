import unittest

from debug_0 import Debug0

class TestDebug0(unittest.TestCase):
    def test_debug_0(self):
        with Debug0.from_file('src/fixed_struct.bin') as r:
            r._read()

            self.assertEqual(r.one, 0x50)
            self.assertEqual(r.array_of_ints, [0x41, 0x43, 0x4b])

            self.assertDictEqual(r._debug['one'], {
                'start': 0,
                'end': 1,
            })
            self.assertDictEqual(r._debug['array_of_ints'], {
                'start': 1,
                'end': 4,
                'arr': [
                    {'start': 1, 'end': 2},
                    {'start': 2, 'end': 3},
                    {'start': 3, 'end': 4},
                ]
            })
