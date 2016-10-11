import unittest

from buffered_struct import BufferedStruct

class TestBufferedStruct(unittest.TestCase):
    def test_buffered_struct(self):
        with BufferedStruct.from_file("src/buffered_struct.bin") as r:

            self.assertEqual(r.len1, 0x10)
            self.assertEqual(r.block1.number1, 0x42)
            self.assertEqual(r.block1.number2, 0x43)
            self.assertEqual(r.len2, 0x8)
            self.assertEqual(r.block2.number1, 0x44)
            self.assertEqual(r.block2.number2, 0x45)
            self.assertEqual(r.finisher, 0xee)
