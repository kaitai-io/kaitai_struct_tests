import unittest

from params_call_short import ParamsCallShort

class TestParamsCallShort(unittest.TestCase):
    def test_params_call_short(self):
        r = ParamsCallShort.from_file("src/term_strz.bin")

        self.assertEqual(r.buf1.body, "foo|b")
        self.assertEqual(r.buf2.body, "ar|ba")
        self.assertEqual(r.buf2.trailer, 0x7a)
