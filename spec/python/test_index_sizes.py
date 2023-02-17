# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from index_sizes import IndexSizes

class TestIndexSizes(unittest.TestCase):
    def test_index_sizes(self):
        with IndexSizes.from_file('src/index_sizes.bin') as r:

            self.assertEqual(r.qty, 3)
            self.assertEqual(r.sizes[0], 1)
            self.assertEqual(r.sizes[1], 8)
            self.assertEqual(r.sizes[2], 4)
            self.assertEqual(r.bufs[0], u"A")
            self.assertEqual(r.bufs[1], u"BBBBBBBB")
            self.assertEqual(r.bufs[2], u"CCCC")
