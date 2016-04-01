import unittest

from position_in_seq import PositionInSeq

class TestPositionInSeq(unittest.TestCase):
    def test_position_in_seq(self):
        r = PositionInSeq.from_file("src/position_in_seq.bin")

        self.assertEquals(r.numbers, [1, 2, 3])
