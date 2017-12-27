import unittest

from imports_rel_1 import ImportsRel1

class TestImportsRel1(unittest.TestCase):
    def test_imports_rel_1(self):
        r = ImportsRel1.from_file("src/fixed_struct.bin")

        self.assertEqual(r.one, 0x50)
        self.assertEqual(r.two.one, 0x41)
        self.assertEqual(r.two.two.one, 0x43)
