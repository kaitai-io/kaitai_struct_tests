import unittest
from testformats.opaque_with_param import OpaqueWithParam

class TestOpaqueWithParam(unittest.TestCase):
    def test_opaque_with_param(self):
        with OpaqueWithParam.from_file('src/term_strz.bin') as r:
            self.assertEqual(r.one.buf, "foo|b")
            self.assertEqual(r.one.trailer, 0x61)
