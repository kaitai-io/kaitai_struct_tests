# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from floating_points import FloatingPoints

class TestFloatingPoints(unittest.TestCase):
    def test_floating_points(self):
        with FloatingPoints.from_file('src/floating_points.bin') as r:

            self.assertAlmostEqual(r.single_value, 0.5, 6)
            self.assertAlmostEqual(r.single_value_be, 0.5, 6)
            self.assertAlmostEqual(r.double_value, 0.25, 6)
            self.assertAlmostEqual(r.double_value_be, 0.25, 6)
            self.assertAlmostEqual(r.approximate_value, 1.2345, 6)
            self.assertAlmostEqual(r.single_value_plus_int, 1.5, 6)
            self.assertAlmostEqual(r.single_value_plus_float, 1.0, 6)
            self.assertAlmostEqual(r.double_value_plus_float, 0.3, 6)
