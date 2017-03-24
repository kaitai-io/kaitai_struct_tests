import unittest

from recursive_one import RecursiveOne

class TestRecursiveOne(unittest.TestCase):
    def test_recursive_one(self):
        r = RecursiveOne.from_file("src/fixed_struct.bin")

        self.assertEqual(r.one, 0x50)
        self.assertEqual(r.next.one, 0x41)
        self.assertEqual(r.next.next.one, 0x43)
        self.assertEqual(r.next.next.next.finisher, 0x2d4b)
