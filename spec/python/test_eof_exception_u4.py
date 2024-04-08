from base_test import BaseTest
from eof_exception_u4 import EofExceptionU4

class TestEofExceptionU4(BaseTest):
    def test_eof_exception_u4(self):
        with self.assertRaisesRegex(EOFError, "^requested \d+ bytes, but only \d+ bytes available$"):
            with EofExceptionU4.from_file('src/term_strz.bin') as r:
                pass
