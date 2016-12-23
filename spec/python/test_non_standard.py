import unittest

from non_standard import NonStandard

class TestNonStandard(unittest.TestCase):
    def test_non_standard(self):
        r = NonStandard.from_file("src/fixed_struct.bin")

        self.assertEqual(r.foo, 0x50)
