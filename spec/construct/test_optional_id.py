import unittest

from optional_id import _schema

class TestOptionalId(unittest.TestCase):
    def test_optional_id(self):
        r = _schema.parse_file('src/fixed_struct.bin')

        self.assertEqual(r._unnamed0, 80)
        self.assertEqual(r._unnamed1, 65)
        self.assertEqual(r._unnamed2, b"\x43\x4B\x2D\x31\xFF")
