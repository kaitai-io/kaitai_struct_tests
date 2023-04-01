# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from testformats.repeat_n_term_struct import RepeatNTermStruct

class TestRepeatNTermStruct(unittest.TestCase):
    def test_repeat_n_term_struct(self):
        with RepeatNTermStruct.from_file('src/repeat_until_process.bin') as r:

            self.assertEqual(len(r.records1), 2)
            self.assertEqual(r.records1[0].value, b"\xE8\xBA")
            self.assertEqual(r.records1[1].value, b"")
            self.assertEqual(len(r.records2), 2)
            self.assertEqual(r.records2[0].value, b"\xAA")
            self.assertEqual(r.records2[1].value, b"\xFA\x9E\xB8\xAA")
            self.assertEqual(len(r.records3), 2)
            self.assertEqual(r.records3[0].value, b"\xAA\xAA")
            self.assertEqual(r.records3[1].value, b"")