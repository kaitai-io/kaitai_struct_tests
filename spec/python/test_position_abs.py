import unittest

from position_abs import PositionAbs

class TestPositionAbs(unittest.TestCase):
    def test_position_abs(self):
        r = PositionAbs.from_file("src/position_abs.bin")

        self.assertEquals(r.index_offset, 0x20)
        self.assertEquals(r.index.entry, "foo")
