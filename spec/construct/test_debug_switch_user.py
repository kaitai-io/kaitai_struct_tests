# runs in debug mode, so the _read() needs to be called manually

import unittest

from debug_switch_user import _schema

class TestDebugSwitchUser(unittest.TestCase):
    def test_debug_switch_user(self):
        r = _schema.parse_file('src/nav_parent_switch.bin')
        r._read()

        self.assertEqual(r.code, 1)
        self.assertEqual(r.data.val, -190)
