# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest

from debug_array_user_current_excluded import _schema

class TestDebugArrayUserCurrentExcluded(unittest.TestCase):
    def test_debug_array_user_current_excluded(self):
        r = _schema.parse_file('src/term_strz.bin')
        self.assertEqual(r.array_of_cats[0].meow, b"\x66\x6F\x6F")
        self.assertEqual(r.array_of_cats[1].meow, b"\x7C\x62")
        self.assertEqual(r.array_of_cats[2].meow, b"\x61")
