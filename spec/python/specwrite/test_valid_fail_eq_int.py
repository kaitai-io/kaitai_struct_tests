import unittest
import kaitaistruct
from specwrite.common_spec import CommonSpec

from testwrite.valid_fail_eq_int import ValidFailEqInt

class TestValidFailEqInt(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestValidFailEqInt, self).__init__(*args, **kwargs)
        self.struct_class = ValidFailEqInt
        self.src_filename = 'src/fixed_struct.bin'

    def test_read_write_roundtrip(self):
        self.skipTest("cannot use roundtrip because parsing is expected to fail")

    def test_check_bad_valid_no_io(self):
        r = ValidFailEqInt()
        r.foo = 0x50
        self.assert_check_valid_fail(r)

    def test_check_bad_valid_old_io(self):
        r = ValidFailEqInt.from_file(self.src_filename)
        with self.assertRaises(kaitaistruct.ValidationNotEqualError):
            r._read()
        self.assert_check_valid_fail(r)

    def assert_check_valid_fail(self, r):
        with self.assertRaises(kaitaistruct.ValidationNotEqualError) as cm:
            r._check()

        # NB: the error message must not contain the "at pos X: " part because
        # _check() is not supposed to access `_io` at all (even if it happens
        # to be non-`null`, as in this case)
        self.assertEqual(str(cm.exception), "/seq/0: validation failed: not equal, expected 123, but got 80")
