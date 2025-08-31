import unittest
from testformats.to_string_custom import ToStringCustom

class TestToStringCustom(unittest.TestCase):
    def test_to_string_custom(self):
        with ToStringCustom.from_file('src/term_strz.bin') as r:
            self.assertEqual(str(r), "s1 = foo, s2 = bar")
