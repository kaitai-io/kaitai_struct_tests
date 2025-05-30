# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from eos_exception_bytes import EosExceptionBytes
import kaitaistruct

class TestEosExceptionBytes(unittest.TestCase):
    def test_eos_exception_bytes(self):
        with self.assertRaises(EOFError) as cm:
            with EosExceptionBytes.from_file('src/term_strz.bin') as r:
                pass
        self.assertEqual(str(cm.exception), u"requested 7 bytes, but only 6 bytes available")
