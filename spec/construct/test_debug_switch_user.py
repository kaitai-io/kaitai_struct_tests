# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from debug_switch_user import _schema

class TestDebugSwitchUser(unittest.TestCase):
    def test_debug_switch_user(self):
        r = _schema.parse_file('src/nav_parent_switch.bin')

        self.assertEqual(r.code, 1)
        self.assertEqual(r.data.val, -190)
