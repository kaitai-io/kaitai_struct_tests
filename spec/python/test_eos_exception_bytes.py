import unittest
import kaitaistruct

from eos_exception_bytes import EosExceptionBytes

class TestEosExceptionBytes(unittest.TestCase):
    def test_eos_exception_bytes(self):
        with self.assertRaisesRegexp(EOFError, "^requested \d+ bytes, but only \d+ bytes available$"):
            with EosExceptionBytes.from_file('src/term_strz.bin') as r:
                pass
