import unittest

from repeat_until_sized import RepeatUntilSized

class TestRepeatUntilSized(unittest.TestCase):
    def test_repeat_until_sized(self):
        r = RepeatUntilSized.from_file("src/repeat_until_process.bin")

        self.assertEqual(len(r.records), 3)

        self.assertEqual(r.records[0].marker, 0xe8)
        self.assertEqual(r.records[0].body, 0xaaaaaaba)

        self.assertEqual(r.records[1].marker, 0xfa)
        self.assertEqual(r.records[1].body, 0xaaaab89e)

        self.assertEqual(r.records[2].marker, 0xaa)
        self.assertEqual(r.records[2].body, 0x55555555)
