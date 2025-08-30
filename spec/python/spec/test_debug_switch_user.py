import unittest
from testformats.debug_switch_user import DebugSwitchUser

class TestDebugSwitchUser(unittest.TestCase):
    def test_debug_switch_user(self):
        with DebugSwitchUser.from_file('src/nav_parent_switch.bin') as r:
            # --debug implies --no-auto-read
            r._read()

            self.assertEqual(r.code, 1)
            self.assertEqual(r.data.val, -190)
