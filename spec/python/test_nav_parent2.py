import unittest

from nav_parent2 import NavParent2

class TestNavParent2(unittest.TestCase):
    def test_nav_parent2(self):
        r = NavParent2.from_file("src/nav_parent2.bin")

        self.assertEqual(r.ofs_tags, 8)
        self.assertEqual(r.num_tags, 2)

        self.assertEqual(r.tags[0].name, "RAHC")
        self.assertEqual(r.tags[0].ofs, 0x20)
        self.assertEqual(r.tags[0].num_items, 3)
        self.assertEqual(r.tags[0].tag_content.content, "foo")

        self.assertEqual(r.tags[1].name, "RAHC")
        self.assertEqual(r.tags[1].ofs, 0x23)
        self.assertEqual(r.tags[1].num_items, 6)
        self.assertEqual(r.tags[1].tag_content.content, "barbaz")
