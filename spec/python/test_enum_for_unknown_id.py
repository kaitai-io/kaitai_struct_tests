import unittest

from enum_for_unknown_id import EnumForUnknownId

class TestEnumForUnknownId(unittest.TestCase):
    def test_enum_for_unknown_id(self):
        with EnumForUnknownId.from_file("src/fixed_struct.bin") as r:
            self.assertEqual(r.one, 80)
