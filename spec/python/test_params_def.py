import unittest

from params_def import ParamsDef
from kaitaistruct import KaitaiStream

class TestParamsDef(unittest.TestCase):
    def test_params_def(self):
        io = KaitaiStream(open("src/term_strz.bin", "rb"))
        r = ParamsDef(5, True, io, None, None)

        self.assertEqual(r.buf, "foo|b")
        self.assertEqual(r.trailer, 0x61)

        io.close()
