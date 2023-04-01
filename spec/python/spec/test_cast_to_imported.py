# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from testformats.cast_to_imported import CastToImported

class TestCastToImported(unittest.TestCase):
    def test_cast_to_imported(self):
        with CastToImported.from_file('src/fixed_struct.bin') as r:

            self.assertEqual(r.one.one, 80)
            self.assertEqual(r.one_casted.one, 80)