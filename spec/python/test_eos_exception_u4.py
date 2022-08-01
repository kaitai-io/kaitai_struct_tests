import unittest
import kaitaistruct

from eos_exception_u4 import EosExceptionU4

class TestEosExceptionU4(unittest.TestCase):
    def test_eos_exception_u4(self):
        with self.assertRaisesRegexp(EOFError, "^requested \d+ bytes, but only \d+ bytes available$"):
            with EosExceptionU4.from_file('src/term_strz.bin') as r:
                pass
