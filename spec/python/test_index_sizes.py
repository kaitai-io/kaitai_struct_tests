import unittest

from index_sizes import IndexSizes

class TestIndexSizes(unittest.TestCase):
    def test_index_sizes(self):
        r = IndexSizes.from_file("src/index_sizes.bin")

        self.assertEqual(r.qty, 3)

        self.assertEqual(r.sizes[0], 1)
        self.assertEqual(r.sizes[1], 8)
        self.assertEqual(r.sizes[2], 4)

        self.assertEqual(r.bufs[0], "A")
        self.assertEqual(r.bufs[1], "BBBBBBBB")
        self.assertEqual(r.bufs[2], "CCCC")
