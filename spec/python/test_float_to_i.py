import unittest

from float_to_i import FloatToI

class TestFloatToI(unittest.TestCase):
    def test_float_to_i(self):
        r = FloatToI.from_file("src/floating_points.bin")
        
        self.assertEqual(r.single_value, 0.5)
        self.assertEqual(r.double_value, 0.25)
        
        self.assertEqual(r.single_i, 0)
        self.assertEqual(r.double_i, 0)
        self.assertEqual(r.float1_i, 1)
        self.assertEqual(r.float2_i, 1)
        self.assertEqual(r.float3_i, 1)
        self.assertEqual(r.float4_i, -2)
