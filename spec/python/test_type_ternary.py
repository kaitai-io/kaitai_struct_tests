import unittest

from type_ternary import TypeTernary

class TestTypeTernary(unittest.TestCase):
    def test_type_ternary(self):
        r = TypeTernary.from_file("src/term_strz.bin")

        self.assertEqual(r.dif.value, 0x65)
