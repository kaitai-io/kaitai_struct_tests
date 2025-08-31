import unittest
from testformats.params_def import ParamsDef
from kaitaistruct import KaitaiStream

class TestParamsDef(unittest.TestCase):
    def test_params_def(self):
        io = KaitaiStream(open('src/term_strz.bin', 'rb'))
        with ParamsDef(5, True, io, None, None) as r:
            self.assertEqual(r.buf, "foo|b")
            self.assertEqual(r.trailer, 0x61)
