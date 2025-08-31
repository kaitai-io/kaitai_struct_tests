import unittest
from testformats.imports_circular_a import ImportsCircularA

class TestImportsCircularA(unittest.TestCase):
    def test_imports_circular_a(self):
        with ImportsCircularA.from_file('src/fixed_struct.bin') as r:
            self.assertEqual(r.code, 80)
            self.assertEqual(r.two.initial, 65)
            self.assertEqual(r.two.back_ref.code, 67)
            self.assertEqual(r.two.back_ref.two.initial, 75)
            self.assertFalse(hasattr(r.two.back_ref.two, 'back_ref'))
