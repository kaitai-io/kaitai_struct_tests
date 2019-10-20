import unittest

from eos_exception_bytes import EosExceptionBytes

class TestEosExceptionBytes(unittest.TestCase):
    def test_eos_exception_bytes(self):
        with self.assertRaises(EOFError):
            with EosExceptionBytes.from_file("src/term_strz.bin"):
                pass
