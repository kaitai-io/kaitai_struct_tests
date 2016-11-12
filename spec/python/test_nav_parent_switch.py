import unittest

from nav_parent_switch import NavParentSwitch

class TestNavParentSwitch(unittest.TestCase):
    def test_nav_parent_switch(self):
        r = NavParentSwitch.from_file("src/nav_parent_switch.bin")

        self.assertEquals(r.category, 1)
        self.assertEquals(r.content.foo, 0x42)
        self.assertEquals(r.content.subelement.bar, 0xff)
