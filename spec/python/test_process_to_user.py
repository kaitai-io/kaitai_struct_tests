import unittest

from process_to_user import ProcessToUser

class TestProcessToUser(unittest.TestCase):
    def test_process_to_user(self):
        r = ProcessToUser.from_file("src/process_rotate.bin")

        self.assertEquals(r.buf1.str, "Hello")
