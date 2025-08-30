import unittest
from testformats.eof_exception_bits_le import EofExceptionBitsLe
import kaitaistruct

class TestEofExceptionBitsLe(unittest.TestCase):
    def test_eof_exception_bits_le(self):
        with EofExceptionBitsLe.from_file('src/nav_parent_switch.bin') as r:
            with self.assertRaises(EOFError) as cm:
                r._read()
            self.assertEqual(str(cm.exception), u"requested 3 bytes, but only 2 bytes available")

            self.assertEqual(r._io.pos(), 1)

            self.assertEqual(r.pre_bits, 0b0000001)  # 0b000_0001
            self.assertIn('start', r._debug['fail_bits'])
            self.assertEqual(r._debug['fail_bits']['start'], 1)
            self.assertFalse(hasattr(r, 'fail_bits'))
            self.assertNotIn('end', r._debug['fail_bits'])
