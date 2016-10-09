import unittest

from position_abs import PositionAbs

class TestPositionAbs(unittest.TestCase):
    def test_position_abs(self):
        r = PositionAbs.from_file("src/position_abs.bin")

        self.assertEqual(r.index_offset, 0x20)
        self.assertEqual(r.index.entry, "foo")
