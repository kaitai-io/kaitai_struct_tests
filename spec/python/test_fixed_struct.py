import unittest

from fixed_struct import FixedStruct

class TestFixedStruct(unittest.TestCase):
    def test_fixed_struct(self):
        r = FixedStruct.from_file('src/fixed_struct.bin')

        self.assertEqual(r.header.uint8, 255)
        self.assertEqual(r.header.uint16, 65535)
        self.assertEqual(r.header.uint32, 4294967295)
        self.assertEqual(r.header.uint64, 18446744073709551615)

        self.assertEqual(r.header.sint8, -1)
        self.assertEqual(r.header.sint16, -1)
        self.assertEqual(r.header.sint32, -1)
        self.assertEqual(r.header.sint64, -1)

        self.assertEqual(r.header.uint16le, 66)
        self.assertEqual(r.header.uint32le, 66)
        self.assertEqual(r.header.uint64le, 66)

        self.assertEqual(r.header.sint16le, -66)
        self.assertEqual(r.header.sint32le, -66)
        self.assertEqual(r.header.sint64le, -66)

        self.assertEqual(r.header.uint16be, 66)
        self.assertEqual(r.header.uint32be, 66)
        self.assertEqual(r.header.uint64be, 66)

        self.assertEqual(r.header.sint16be, -66)
        self.assertEqual(r.header.sint32be, -66)
        self.assertEqual(r.header.sint64be, -66)

        r.close()
