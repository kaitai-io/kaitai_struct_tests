# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from str_eos import StrEos

class TestStrEos(unittest.TestCase):
    def test_str_eos(self):
        with StrEos.from_file('src/term_strz.bin') as r:

            self.assertEqual(r.str, u"foo|bar|baz@")
