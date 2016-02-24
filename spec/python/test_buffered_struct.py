import unittest

from buffered_struct import BufferedStruct

class TestBufferedStruct(unittest.TestCase):
    def test_buffered_struct(self):
        r = BufferedStruct.from_file("src/buffered_struct.bin")

        self.assertEquals(r.len1, 0x10)
        self.assertEquals(r.block1.number1, 0x42)
        self.assertEquals(r.block1.number2, 0x43)
        self.assertEquals(r.len2, 0x8)
        self.assertEquals(r.block2.number1, 0x44)
        self.assertEquals(r.block2.number2, 0x45)
        self.assertEquals(r.finisher, 0xee)
