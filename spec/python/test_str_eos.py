import unittest

from str_eos import StrEos

class TestStrEos(unittest.TestCase):
    def test_str_eos(self):
        r = StrEos.from_file("src/term_strz.bin")

        self.assertEquals(r.str, "foo|bar|baz@")
