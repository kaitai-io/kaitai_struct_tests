# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from hello_world import HelloWorld

class TestHelloWorld(unittest.TestCase):
    def test_hello_world(self):
        with HelloWorld.from_file('src/fixed_struct.bin') as r:
            self.assertEqual(r.one, 80)
