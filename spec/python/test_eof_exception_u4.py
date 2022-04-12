import unittest
import kaitaistruct

from eof_exception_u4 import EofExceptionU4

class TestEofExceptionU4(unittest.TestCase):
    def test_eof_exception_u4(self):
        with self.assertRaisesRegexp(EOFError, "requested \d+ bytes, but only \d+ bytes available"):
            with EofExceptionU4.from_file('src/term_strz.bin') as r:
                pass
