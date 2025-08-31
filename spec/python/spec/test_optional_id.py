import unittest
from testformats.optional_id import OptionalId

class TestOptionalId(unittest.TestCase):
    def test_optional_id(self):
        with OptionalId.from_file('src/fixed_struct.bin') as r:
            self.assertEqual(r._unnamed0, 80)
            self.assertEqual(r._unnamed1, 65)
            self.assertEqual(r._unnamed2, b"\x43\x4B\x2D\x31\xFF")
