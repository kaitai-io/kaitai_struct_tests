import io
import unittest
from kaitaistruct import KaitaiStream
from specwrite.common_spec import CommonSpec

from testwrite.process_to_user import ProcessToUser

class TestProcessToUser(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestProcessToUser, self).__init__(*args, **kwargs)
        self.struct_class = ProcessToUser
        self.src_filename = 'src/process_rotate.bin'

    def test_process_to_user(self):
        # NOTE: unlike the automatic roundtrip test, the `_raw_*` fields are unset in this
        # manual test, so "cheating" by just writing them is impossible

        r = ProcessToUser()

        buf1 = ProcessToUser.JustStr(None, r, r._root)
        buf1.str = "Hello"
        buf1._check()
        r.buf1 = buf1

        r._check()

        with io.open('src/process_rotate.bin', 'rb') as f:
            expected = f.read(5)
        self.assertEqual(len(expected), 5)

        ks_io = KaitaiStream(io.BytesIO(bytearray(len(expected))))
        r._write(ks_io)

        actual = ks_io.to_byte_array()

        self.assert_byte_array_equal(actual, expected)
