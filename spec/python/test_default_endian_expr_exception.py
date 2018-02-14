import unittest

from default_endian_expr_exception import DefaultEndianExprException
import kaitaistruct

class TestDefaultEndianExprException(unittest.TestCase):
    def test_default_endian_expr_exception(self):
        with self.assertRaises(Exception):
            with DefaultEndianExprException.from_file("src/endian_expr.bin"):
                pass
