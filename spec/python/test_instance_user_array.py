import unittest

from instance_user_array import InstanceUserArray

class TestInstanceUserArray(unittest.TestCase):
    def test_instance_user_array(self):
        r = InstanceUserArray.from_file("src/instance_std_array.bin")

        self.assertEquals(r.ofs, 0x10)
        self.assertEquals(r.qty_entries, 3)
        self.assertEquals(r.entry_size, 4)

        self.assertEquals(r.entries[0].word1, 0x1111)
        self.assertEquals(r.entries[0].word2, 0x1111)
        self.assertEquals(r.entries[1].word1, 0x2222)
        self.assertEquals(r.entries[1].word2, 0x2222)
        self.assertEquals(r.entries[2].word1, 0x3333)
        self.assertEquals(r.entries[2].word2, 0x3333)
