import unittest
from testformats.debug_array_user_current_excluded import DebugArrayUserCurrentExcluded

class TestDebugArrayUserCurrentExcluded(unittest.TestCase):
    def test_debug_array_user_current_excluded(self):
        with DebugArrayUserCurrentExcluded.from_file('src/term_strz.bin') as r:
            # --debug implies --no-auto-read
            r._read()

            self.assertEqual(r.array_of_cats[0].meow, b"\x66\x6F\x6F")
            self.assertEqual(r.array_of_cats[1].meow, b"\x7C\x62")
            self.assertEqual(r.array_of_cats[2].meow, b"\x61")
