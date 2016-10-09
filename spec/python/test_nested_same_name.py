import unittest

from nested_same_name import NestedSameName

class TestNestedSameName(unittest.TestCase):
    def test_nested_same_name(self):
        r = NestedSameName.from_file("src/repeat_n_struct.bin")

        self.assertEqual(r.main_data.main_size, 2)
        self.assertEqual(r.main_data.foo.data, "\x10\0\0\0")
