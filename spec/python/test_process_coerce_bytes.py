import unittest

from process_coerce_bytes import ProcessCoerceBytes

class TestProcessCoerceBytes(unittest.TestCase):
    def test_process_coerce_bytes(self):
        r = ProcessCoerceBytes.from_file("src/process_coerce_bytes.bin")

        self.assertEqual(r.records[0].flag, 0)
        self.assertEqual(r.records[0].buf, b"AAAA")
        self.assertEqual(r.records[1].flag, 1)
        self.assertEqual(r.records[1].buf, b"BBBB")
