import unittest
from eof_exception_bytes import EofExceptionBytes
import kaitaistruct

class TestEofExceptionBytes(unittest.TestCase):
    def test_eof_exception_bytes(self):
        with self.assertRaisesRegexp(EOFError, "^requested \d+ bytes, but only \d+ bytes available$"):
            with EofExceptionBytes.from_file('src/term_strz.bin') as r:
                pass
