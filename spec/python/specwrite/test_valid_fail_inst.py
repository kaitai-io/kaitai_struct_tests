import unittest
import kaitaistruct
from specwrite.common_spec import CommonSpec

from testwrite.valid_fail_inst import ValidFailInst

class TestValidFailInst(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestValidFailInst, self).__init__(*args, **kwargs)
        self.struct_class = ValidFailInst
        self.src_filename = 'src/fixed_struct.bin'

    def test_read_write_roundtrip(self):
        self.skipTest("cannot use roundtrip because parsing is expected to fail")

    def test_check_bad_valid_no_io(self):
        r = ValidFailInst()
        r.a = 0x50
        r.inst = 0x31
        self.assert_check_valid_fail(r)

    def test_check_bad_valid_old_io(self):
        r = ValidFailInst.from_file(self.src_filename)
        with self.assertRaises(kaitaistruct.ValidationNotEqualError):
            r._read()
        self.assert_check_valid_fail(r)

    def assert_check_valid_fail(self, r):
        with self.assertRaises(kaitaistruct.ValidationNotEqualError) as cm:
            r._check()

        # NB: the error message must not contain the "at pos X: " part because
        # _check() is not supposed to access `_io` at all (even if it happens
        # to be non-`null`, as in this case)
        self.assertEqual(str(cm.exception), "/instances/inst: validation failed: not equal, expected 80, but got 49")
