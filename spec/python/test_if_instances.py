import unittest

from if_instances import IfInstances

class TestIfInstances(unittest.TestCase):
    def test_if_instances(self):
        r = IfInstances.from_file("src/fixed_struct.bin")

        self.assertIsNone(r.never_happens)
