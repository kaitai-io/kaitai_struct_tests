import io
import unittest
from kaitaistruct import KaitaiStream
from specwrite.common_spec import CommonSpec

from testwrite.cast_to_top import CastToTop

class TestCastToTop(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestCastToTop, self).__init__(*args, **kwargs)
        self.struct_class = CastToTop
        self.src_filename = 'src/fixed_struct.bin'

    def test_inst_disabled(self):
        r = CastToTop()
        r.code = 0x77

        s = CastToTop(None, r, r._root)
        s.code = 0xa8
        # `cast_to_top.ksy` is an infinitely recursive format, so it's necessary to break
        # the infinite recursion by disabling the `header` instance here.
        s.header__enabled = False
        s._check()

        r.header = s
        r._check()

        ks_io = KaitaiStream(io.BytesIO(bytearray(2)))
        r._write(ks_io)

        actual = ks_io.to_byte_array()
        self.assert_byte_array_equal(actual, b"\x77\xa8")
