import unittest

from user_type import UserType

class TestUserType(unittest.TestCase):
    def test_user_type(self):
        r = UserType.from_file("src/repeat_until_s4.bin")

        self.assertEqual(r.one.width, 0x42)
        self.assertEqual(r.one.height, 0x1337)
