from base_test import BaseTest
from eos_exception_u4 import EosExceptionU4

class TestEosExceptionU4(BaseTest):
    def test_eos_exception_u4(self):
        with self.assertRaisesRegex(EOFError, r"^requested \d+ bytes, but only \d+ bytes available$"):
            with EosExceptionU4.from_file('src/term_strz.bin') as r:
                pass
