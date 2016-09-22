import unittest

from repeat_n_strz_double import RepeatNStrzDouble

class TestRepeatNStrzDouble(unittest.TestCase):
    def test_repeat_n_strz_double(self):
        r = RepeatNStrzDouble.from_file("src/repeat_n_strz.bin")

        self.assertEquals(r.qty, 2)
        self.assertEquals(r.lines1, ["foo"])
        self.assertEquals(r.lines2, ["bar"])
