import unittest
from testformats.debug_array_user import DebugArrayUser

class TestDebugArrayUser(unittest.TestCase):
    def test_debug_array_user(self):
        with DebugArrayUser.from_file('src/fixed_struct.bin') as r:
            # --debug implies --no-auto-read
            r._read()

            self.assertEqual(r.one_cat.meow, 80)
            self.assertEqual(len(r.array_of_cats), 3)
            self.assertEqual(r.array_of_cats[0].meow, 65)
            self.assertEqual(r.array_of_cats[1].meow, 67)
            self.assertEqual(r.array_of_cats[2].meow, 75)
