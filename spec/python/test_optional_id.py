import unittest

from optional_id import OptionalId

class TestOptionalId(unittest.TestCase):
    def test_optional_id(self):
        r = OptionalId.from_file("src/fixed_struct.bin")

        self.assertEqual(r._unnamed0, 0x50)
        self.assertEqual(r._unnamed1, 0x41)
        self.assertEqual(r._unnamed2, b"\x43\x4b\x2d\x31\xff")
