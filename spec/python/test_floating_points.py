import unittest

from floating_points import FloatingPoints

class TestFloatingPoints(unittest.TestCase):
    def test_floating_points(self):
        r = FloatingPoints.from_file("src/floating_points.bin")

        self.assertAlmostEqual(r.single_value, 1.2345)
        self.assertAlmostEqual(r.single_value_be, 1.2345)
        self.assertAlmostEqual(r.double_value, 123.456)
        self.assertAlmostEqual(r.double_value_be, 123.456)

        self.assertAlmostEqual(r.single_value_plus_int, 2.2345)
        self.assertAlmostEqual(r.single_value_plus_float, 1.7345)
        self.assertAlmostEqual(r.double_value_plus_float, 123.506)
