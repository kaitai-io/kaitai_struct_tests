import unittest

from process_rotate import ProcessRotate

class TestProcessRotate(unittest.TestCase):
    def test_process_rotate(self):
        r = ProcessRotate.from_file("src/process_rotate.bin")

        self.assertEqual(r.buf1, "Hello")
        self.assertEqual(r.buf2, "World")
        self.assertEqual(r.buf3, "There")
