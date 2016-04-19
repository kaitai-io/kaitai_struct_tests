import unittest

from process_rotate import ProcessRotate

class TestProcessRotate(unittest.TestCase):
    def test_process_rotate(self):
        r = ProcessRotate.from_file("src/process_rotate.bin")

        self.assertEquals(r.buf1, "Hello")
        self.assertEquals(r.buf2, "World")
        self.assertEquals(r.buf3, "There")
