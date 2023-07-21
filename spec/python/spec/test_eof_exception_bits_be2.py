import unittest
import kaitaistruct

from testformats.eof_exception_bits_be2 import EofExceptionBitsBe2

class TestEofExceptionBitsBe2(unittest.TestCase):
    def test_eof_exception_bits_be2(self):
        with EofExceptionBitsBe2.from_file('src/nav_parent_switch.bin') as r:
            with self.assertRaisesRegexp(EOFError, u"^requested 3 bytes, but only 2 bytes available$"):
                r._read()

            self.assertEqual(r._io.pos(), 1)

            self.assertEqual(r.pre_bits, 0x01)
            self.assertEqual(r._debug['fail_bits']['start'], 1)
            self.assertFalse(hasattr(r, 'fail_bits'))
            self.assertFalse(hasattr(r._debug['fail_bits'], 'end'))
