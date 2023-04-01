# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from testformats.instance_in_repeat_until import InstanceInRepeatUntil

class TestInstanceInRepeatUntil(unittest.TestCase):
    def test_instance_in_repeat_until(self):
        with InstanceInRepeatUntil.from_file('src/repeat_until_s4.bin') as r:

            self.assertEqual(len(r.entries), 5)
            self.assertEqual(r.entries[0], 66)
            self.assertEqual(r.entries[1], 0)
            self.assertEqual(r.entries[2], 4919)
            self.assertEqual(r.entries[3], 0)
            self.assertEqual(r.entries[4], -1)