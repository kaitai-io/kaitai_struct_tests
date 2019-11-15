import unittest

from switch_repeat_expr import SwitchRepeatExpr

class TestSwitchRepeatExpr(unittest.TestCase):
    def test_switch_repeat_expr(self):
        r = SwitchRepeatExpr.from_file("src/switch_tlv.bin")

        self.assertEqual(r.code, 0x11)
        self.assertEqual(r.size, 9)
        self.assertEqual(r.body[0].first, b"Stuff\0Me\0")
