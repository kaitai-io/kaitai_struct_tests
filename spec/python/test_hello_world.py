import unittest

from hello_world import HelloWorld

class TestHelloWorld(unittest.TestCase):
    def test_hello_world(self):
        r = HelloWorld.from_file('src/fixed_struct.bin')
        self.assertEqual(r.one, 0x50)
        r.close()
