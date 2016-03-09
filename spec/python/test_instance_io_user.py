import unittest

from instance_io_user import InstanceIoUser

class TestInstanceIoUser(unittest.TestCase):
    def test_instance_io_user(self):
        r = InstanceIoUser.from_file("src/instance_io.bin")

        self.assertEquals(r.qty_entries, 3)

        self.assertEquals(r.entries[0].name, "the")
        self.assertEquals(r.entries[1].name, "rainy")
        self.assertEquals(r.entries[2].name, "day it is")
