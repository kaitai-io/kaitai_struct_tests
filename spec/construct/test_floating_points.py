import unittest

from floating_points import _schema

class TestFloatingPoints(unittest.TestCase):
    def test_floating_points(self):
        r = _schema.parse_file("src/floating_points.bin")

        prec = 6

        self.assertEqual(r.single_value, 0.5)
        self.assertEqual(r.single_value_be, 0.5)
        self.assertEqual(r.double_value, 0.25)
        self.assertEqual(r.double_value_be, 0.25)

        self.assertAlmostEqual(r.approximate_value, 1.2345, prec)

        self.assertAlmostEqual(r.single_value_plus_int, 1.5, prec)
        self.assertAlmostEqual(r.single_value_plus_float, 1.0, prec)
        self.assertAlmostEqual(r.double_value_plus_float, 0.3, prec)
