# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
import kaitaistruct

from valid_fail_regex import ValidFailRegex

class TestValidFailRegex(unittest.TestCase):
    def test_valid_fail_regex(self):
        with self.assertRaises(kaitaistruct.ValidationRegexMatchError):
            with ValidFailRegex.from_file('src/fixed_struct.bin') as r:
                pass
