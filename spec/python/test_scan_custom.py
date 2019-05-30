import unittest

from scan_custom import ScanCustom

class TestMyCustomScan(unittest.TestCase):
    def test_scan_custom(self):
        with ScanCustom.from_file('src/term_strz.bin') as r:
            self.assertEqual(r.s1, b"\x66\x6F\x6F\x7c")
