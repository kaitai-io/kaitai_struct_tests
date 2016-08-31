import unittest

from nested_types import NestedTypes

class TestNestedTypes(unittest.TestCase):
    def test_nested_types(self):
        r = NestedTypes.from_file("src/fixed_struct.bin")

        self.assertEquals(r.one.typed_at_root.value_b, 80)
        self.assertEquals(r.one.typed_here.value_c, 65)
        self.assertEquals(r.two.value_b, 67)
