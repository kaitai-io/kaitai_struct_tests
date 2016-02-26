import unittest

from repeat_n_struct import RepeatNStruct

class TestRepeatNStruct(unittest.TestCase):
    def test_repeat_n_struct(self):
        r = RepeatNStruct.from_file("src/repeat_n_struct.bin")

        self.assertEquals(len(r.chunks), 2)
        self.assertEquals(r.chunks[0].offset, 0x10)
        self.assertEquals(r.chunks[0].len, 0x2078)
        self.assertEquals(r.chunks[1].offset, 0x2088)
        self.assertEquals(r.chunks[1].len, 0xf)
