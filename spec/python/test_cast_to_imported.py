import unittest

from cast_to_imported import CastToImported

class TestCastToImported(unittest.TestCase):
    def test_cast_to_imported(self):
        r = CastToImported.from_file("src/fixed_struct.bin")

        self.assertEqual(r.one.one, 0x50)
        self.assertEqual(r.one_casted.one, 0x50)
