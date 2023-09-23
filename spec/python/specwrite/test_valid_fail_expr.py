import unittest
import kaitaistruct
from specwrite.common_spec import CommonSpec

from testwrite.valid_fail_expr import ValidFailExpr

class TestValidFailExpr(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestValidFailExpr, self).__init__(*args, **kwargs)
        self.struct_class = ValidFailExpr
        self.src_filename = 'src/nav_parent_switch.bin'

    def test_read_write_roundtrip(self):
        self.skipTest("cannot use roundtrip because parsing is expected to fail")

    def test_check_bad_valid_no_io(self):
        r = ValidFailExpr()
        r.foo = 1
        r.bar = -190
        self.assert_check_valid_fail(r)

    def test_check_bad_valid_old_io(self):
        r = ValidFailExpr.from_file(self.src_filename)
        with self.assertRaises(kaitaistruct.ValidationExprError):
            r._read()
        self.assert_check_valid_fail(r)

    def assert_check_valid_fail(self, r):
        with self.assertRaises(kaitaistruct.ValidationExprError) as cm:
            r._check()

        # NB: the error message must not contain the "at pos X: " part because
        # _check() is not supposed to access `_io` at all (even if it happens
        # to be non-`null`, as in this case)
        self.assertEqual(str(cm.exception), "/seq/1: validation failed: not matching the expression, got -190")
