import unittest

from eof_exception_bytes import EofExceptionBytes

class TestEofExceptionBytes(unittest.TestCase):
    def test_eof_exception_bytes(self):
        with self.assertRaises(EOFError):
            with EofExceptionBytes.from_file("src/term_strz.bin"):
                pass
