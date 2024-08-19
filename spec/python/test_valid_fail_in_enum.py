# Autogenerated from KST: please remove this line if doing any edits by hand!

import unittest
from valid_fail_in_enum import ValidFailInEnum
import kaitaistruct

class TestValidFailInEnum(unittest.TestCase):
    def test_valid_fail_in_enum(self):
        with self.assertRaises(kaitaistruct.ValidationNotInEnumError):
            with ValidFailInEnum.from_file('src/enum_0.bin') as r:
                pass
