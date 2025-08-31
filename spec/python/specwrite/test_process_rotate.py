import io
import unittest
from kaitaistruct import ConsistencyError, KaitaiStream
from specwrite.common_spec import CommonSpec

from testwrite.process_rotate import ProcessRotate

class TestProcessRotate(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestProcessRotate, self).__init__(*args, **kwargs)
        self.struct_class = ProcessRotate
        self.src_filename = 'src/process_rotate.bin'

    def test_process_rotate(self):
        # NOTE: unlike the automatic roundtrip test, the `_raw_*` fields are unset in this
        # manual test, so "cheating" by just writing them is impossible

        r = ProcessRotate()

        r.buf1 = b"Hello"
        r.buf2 = b"World"
        r.key = 1
        r.buf3 = b"There"

        self.assert_equal_to_full_file(r, 'src/process_rotate.bin')

    def test_check_size_mismatch_check(self):
        r = ProcessRotate()

        r.buf1 = b"Hello"
        r.buf2 = b"Way too long"

        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: buf2,"):
            r._check()

    def test_check_size_mismatch_check_or_write(self):
        r = ProcessRotate()

        r.buf1 = b"Hello"
        r.buf2 = b"Way too long"
        r.key = 1
        r.buf3 = b"There"

        len_io = (
            5 +  # buf1
            5 +  # buf2
            1 +  # key
            5  # buf3
        )
        ks_io = KaitaiStream(io.BytesIO(bytearray(len_io)))

        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: buf2,"):
            r._check()

            # It would be more user-friendly if the size mismatch of `buf2` was caught already in
            # _check(), as tested by the previous test case test_check_size_mismatch_check() (this
            # is possible because it's known that `process: rol/ror(...)` preserve the input length,
            # so the length of `buf2` is the same as the length of `_raw_buf2` which actually gets
            # written). But if the compiler implementation doesn't want to distinguish among
            # `process` types here, just a check in _write() is probably acceptable as well.
            r._write(ks_io)
