import unittest

from repeat_until_s4 import RepeatUntilS4

class TestRepeatUntilS4(unittest.TestCase):
    def test_repeat_until_s4(self):
        r = RepeatUntilS4.from_file("src/repeat_until_s4.bin")

        self.assertEqual(r.entries, [0x42, 0x1337, -251658241, -1])
        self.assertEqual(r.afterall, "foobar")
