# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from testformats.non_standard import NonStandard

class TestNonStandard(unittest.TestCase):
    def test_non_standard(self):
        with NonStandard.from_file('src/fixed_struct.bin') as r:

            self.assertEqual(r.foo, 80)