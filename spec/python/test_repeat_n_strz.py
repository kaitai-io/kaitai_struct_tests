import unittest

from repeat_n_strz import RepeatNStrz

class TestRepeatNStrz(unittest.TestCase):
    def test_repeat_n_strz(self):
        with RepeatNStrz.from_file("src/repeat_n_strz.bin") as r:

            self.assertEqual(r.qty, 2)
            self.assertEqual(r.lines, ["foo", "bar"])
