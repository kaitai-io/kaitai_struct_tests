import unittest
from testformats.debug_0 import Debug0

class TestDebug0(unittest.TestCase):
    def test_debug_0(self):
        with Debug0.from_file('src/fixed_struct.bin') as r:
            r._read()

            self.assertEqual(r.one, 80)
            self.assertEqual(r.array_of_ints, [65, 67, 75])
            self.assertEqual(r._unnamed2, 45)

            # At the time of writing, `_debug` is of type `collections.defaultdict`
            # (a subclass of `dict`), not plain `dict`.
            # `self.assertEqual(r._debug, { ... })` would still work, but it
            # wouldn't show a nice deep diff if the actual and expected values
            # didn't match.
            self.assertIsInstance(r._debug, dict)
            self.assertEqual(dict(r._debug), {
                'one': {
                    'start': 0,
                    'end': 1,
                },
                'array_of_ints': {
                    'start': 1,
                    'end': 4,
                    'arr': [
                        {'start': 1, 'end': 2},
                        {'start': 2, 'end': 3},
                        {'start': 3, 'end': 4},
                    ],
                },
                '_unnamed2': {
                    'start': 4,
                    'end': 5,
                },
            })
