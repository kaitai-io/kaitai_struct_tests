import unittest

from process_coerce_switch import ProcessCoerceSwitch

class TestProcessCoerceSwitch(unittest.TestCase):
    def test_process_coerce_switch(self):
        r = ProcessCoerceSwitch.from_file("src/process_coerce_switch.bin")

        self.assertEqual(r.buf_type, 0)
        self.assertEqual(r.flag, 0)
        self.assertEqual(r.buf.bar, b"AAAA")
