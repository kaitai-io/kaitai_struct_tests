import unittest

from str_eos import StrEos

class TestStrEos(unittest.TestCase):
    def test_str_eos(self):
        with StrEos.from_file("src/term_strz.bin") as r:

            self.assertEqual(r.str, "foo|bar|baz@")
