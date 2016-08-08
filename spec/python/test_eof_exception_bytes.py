import unittest

from eof_exception_bytes import EofExceptionBytes

class TestEofExceptionBytes(unittest.TestCase):
    def test_eof_exception_bytes(self):
        self.assertRaises(EOFError, EofExceptionBytes.from_file, "src/term_strz.bin")
