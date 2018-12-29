import unittest

from debug_array_user import DebugArrayUser

class TestDebugArrayUser(unittest.TestCase):
    def test_debug_array_user(self):
        with DebugArrayUser.from_file('src/fixed_struct.bin') as r:
            # --debug implies --no-auto-read            
            r._read()
            
            self.assertEqual(r.one_cat.meow, 0x50)
            self.assertEqual(r.array_of_cats[0].meow, 0x41)
            self.assertEqual(r.array_of_cats[1].meow, 0x43)
            self.assertEqual(r.array_of_cats[2].meow, 0x4b)
