import unittest
import kaitaistruct

from testformats.eof_exception_bits_le import EofExceptionBitsLe

class TestEofExceptionBitsLe(unittest.TestCase):
    def test_eof_exception_bits_le(self):
        with EofExceptionBitsLe.from_file('src/nav_parent_switch.bin') as r:
            with self.assertRaisesRegexp(EOFError, u"^requested 3 bytes, but only 2 bytes available$"):
                r._read()

            self.assertEqual(r._io.pos(), 1)

            self.assertEqual(r.pre_bits, 0b0000001)  # 0b000_0001
            self.assertEqual(r._debug['fail_bits']['start'], 1)
            self.assertFalse(hasattr(r, 'fail_bits'))
            self.assertFalse(hasattr(r._debug['fail_bits'], 'end'))
