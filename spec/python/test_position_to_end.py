import unittest

from position_to_end import PositionToEnd

class TestPositionToEnd(unittest.TestCase):
    def test_position_to_end(self):
        r = PositionToEnd.from_file("src/position_to_end.bin")

        self.assertEqual(r.index.foo, 0x42)
        self.assertEqual(r.index.bar, 0x1234)
