import unittest

from process_rotate import ProcessRotate

class TestProcessRotate(unittest.TestCase):
    def test_process_rotate(self):
        with ProcessRotate.from_file("src/process_rotate.bin") as r:

            self.assertEqual(r.buf1, b"Hello")
            self.assertEqual(r.buf2, b"World")
            self.assertEqual(r.buf3, b"There")
