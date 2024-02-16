import unittest

from str_literals import _schema

class TestStrLiterals(unittest.TestCase):
    def test_str_literals(self):
        r = _schema.parse_file('src/fixed_struct.bin')

        self.assertEqual(self.str_to_arr(r.complex_str), [0, 1, 2, 7, 8, 10, 13, 9, 11, 12, 27, 61, 7, 10, 36, 9787])
        self.assertEqual(self.str_to_arr(r.double_quotes), [34, 34, 34])
        self.assertEqual(self.str_to_arr(r.backslashes), [92, 92, 92])
        self.assertEqual(self.str_to_arr(r.octal_eatup), [0, 50, 50])
        self.assertEqual(self.str_to_arr(r.octal_eatup2), [2, 50])

    def str_to_arr(self, s):
        return [ord(c) for c in s]
