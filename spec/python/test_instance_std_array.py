import unittest

from instance_std_array import InstanceStdArray

class TestInstanceStdArray(unittest.TestCase):
    def test_instance_std_array(self):
        r = InstanceStdArray.from_file("src/instance_std_array.bin")

        self.assertEqual(r.ofs, 0x10)
        self.assertEqual(r.qty_entries, 3)
        self.assertEqual(r.entry_size, 4)

        self.assertEqual(r.entries, [
            "\x11\x11\x11\x11",
            "\x22\x22\x22\x22",
            "\x33\x33\x33\x33",
        ])
