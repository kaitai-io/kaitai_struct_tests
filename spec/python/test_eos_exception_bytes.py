from base_test import BaseTest
from eos_exception_bytes import EosExceptionBytes

class TestEosExceptionBytes(BaseTest):
    def test_eos_exception_bytes(self):
        with self.assertRaisesRegex(EOFError, r"^requested \d+ bytes, but only \d+ bytes available$"):
            with EosExceptionBytes.from_file('src/term_strz.bin') as r:
                pass
