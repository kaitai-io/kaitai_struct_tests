import io
import unittest
from kaitaistruct import ConsistencyError, KaitaiStream
from specwrite.common_spec import CommonSpec

from testwrite.instance_std import InstanceStd

class TestInstanceStd(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestInstanceStd, self).__init__(*args, **kwargs)
        self.struct_class = InstanceStd
        self.src_filename = 'src/str_encodings.bin'

    def test_check_shorter_header(self):
        r = InstanceStd()
        r.header = "1234"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: header,"):
            r._check()

    def test_check_longer_header(self):
        r = InstanceStd()
        r.header = "123456"
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: header,"):
            r._check()

    def test_check_empty_header_via_dump(self):
        r = InstanceStd()
        r.header = ""
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: header,"):
            # calls all _check() methods
            CommonSpec.Base.dump_struct(r)

    def test_write(self):
        r = InstanceStd()
        r.header = u"Hello"

        # see <CommonSpec.Base>.test_read_write_roundtrip
        orig_dump = CommonSpec.Base.dump_struct(r)

        ks_io = KaitaiStream(io.BytesIO(bytearray(2 + 5)))
        r._write(ks_io)
        ks_io.seek(0)

        new_r = InstanceStd(ks_io)
        new_r._read()

        new_dump = CommonSpec.Base.dump_struct(new_r)

        self.assertEqual(new_dump, orig_dump)
