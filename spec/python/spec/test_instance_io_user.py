# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from testformats.instance_io_user import InstanceIoUser

class TestInstanceIoUser(unittest.TestCase):
    def test_instance_io_user(self):
        with InstanceIoUser.from_file('src/instance_io.bin') as r:

            self.assertEqual(r.qty_entries, 3)
            self.assertEqual(r.entries[0].name, u"the")
            self.assertEqual(r.entries[1].name, u"rainy")
            self.assertEqual(r.entries[2].name, u"day it is")