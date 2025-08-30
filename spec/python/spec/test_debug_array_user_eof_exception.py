import unittest
from testformats.debug_array_user_eof_exception import DebugArrayUserEofException

class TestDebugArrayUserEofException(unittest.TestCase):
    def test_debug_array_user_eof_exception(self):
        with DebugArrayUserEofException.from_file('src/nav_parent_codes.bin') as r:
            with self.assertRaises(EOFError):
                # --debug implies --no-auto-read
                r._read()

            self.assertEqual(r.one_cat.meow, 3)
            self.assertEqual(r.one_cat.chirp, 73)
            self.assertEqual(len(r.array_of_cats), 3)
            self.assertEqual(r.array_of_cats[0].meow, 49)
            self.assertEqual(r.array_of_cats[0].chirp, 50)
            self.assertEqual(r.array_of_cats[1].meow, 51)
            self.assertEqual(r.array_of_cats[1].chirp, 66)
            self.assertEqual(r.array_of_cats[2].meow, 98)
