import unittest

from imports_circular_a import ImportsCircularA

class TestImportsCircularA(unittest.TestCase):
    def test_imports_circular_a(self):
        r = ImportsCircularA.from_file("src/fixed_struct.bin")

        self.assertEqual(r.code, 0x50)
        self.assertEqual(r.two.initial, 0x41)
        self.assertEqual(r.two.back_ref.code, 0x43)
        self.assertEqual(r.two.back_ref.two.initial, 0x4b)
        self.assertFalse(hasattr(r.two.back_ref.two, 'back_ref'))
