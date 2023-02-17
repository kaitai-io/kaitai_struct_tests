# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from process_coerce_usertype1 import ProcessCoerceUsertype1

class TestProcessCoerceUsertype1(unittest.TestCase):
    def test_process_coerce_usertype1(self):
        with ProcessCoerceUsertype1.from_file('src/process_coerce_bytes.bin') as r:

            self.assertEqual(r.records[0].flag, 0)
            self.assertEqual(r.records[0].buf.value, 1094795585)
            self.assertEqual(r.records[1].flag, 1)
            self.assertEqual(r.records[1].buf.value, 1111638594)
