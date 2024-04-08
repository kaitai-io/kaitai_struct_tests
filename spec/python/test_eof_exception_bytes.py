from base_test import BaseTest
from eof_exception_bytes import EofExceptionBytes

class TestEofExceptionBytes(BaseTest):
    def test_eof_exception_bytes(self):
        with self.assertRaisesRegex(EOFError, "^requested \d+ bytes, but only \d+ bytes available$"):
            with EofExceptionBytes.from_file('src/term_strz.bin') as r:
                pass
