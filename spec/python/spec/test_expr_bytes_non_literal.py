import unittest
from testformats.expr_bytes_non_literal import ExprBytesNonLiteral
from kaitaistruct import KaitaiStream

class TestExprBytesNonLiteral(unittest.TestCase):
    def test_expr_bytes_non_literal(self):
        with ExprBytesNonLiteral.from_file('src/enum_negative.bin') as r:
            self.assertEqual(len(r.calc_bytes), 2)
            self.assertEqual(KaitaiStream.byte_array_index(r.calc_bytes, 0), 255)
            self.assertEqual(KaitaiStream.byte_array_index(r.calc_bytes, 1), 1)
