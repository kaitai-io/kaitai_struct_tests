import unittest

from repeat_eos_struct import RepeatEosStruct

class TestRepeatEosStruct(unittest.TestCase):
    def test_repeat_eos_struct(self):
        r = RepeatEosStruct.from_file("src/repeat_eos_struct.bin")

        self.assertEquals(len(r.chunks), 2)
        self.assertEquals(r.chunks[0].offset, 0)
        self.assertEquals(r.chunks[0].len, 0x42)
        self.assertEquals(r.chunks[1].offset, 0x42)
        self.assertEquals(r.chunks[1].len, 0x815)
