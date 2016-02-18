import unittest

from repeat_eos_u4 import RepeatEosU4

class TestRepeatEosU4(unittest.TestCase):
    def test_repeat_eos_u4(self):
        r = RepeatEosU4.from_file('src/repeat_eos_struct.bin')

        self.assertEquals(r.numbers, [0, 0x42, 0x42, 0x815])
