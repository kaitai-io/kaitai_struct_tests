import unittest
from kaitaistruct import ConsistencyError
from specwrite.common_spec import CommonSpec

from testwrite.repeat_n_strz import RepeatNStrz

class TestRepeatNStrz(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestRepeatNStrz, self).__init__(*args, **kwargs)
        self.struct_class = RepeatNStrz
        self.src_filename = 'src/repeat_n_strz.bin'

    def test_check_mismatch(self):
        r = RepeatNStrz()

        r.qty = 7
        r.lines = ["foo", "bar"]

        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: lines,"):
            r._check()

    def test_check_null(self):
        r = RepeatNStrz()
        r.qty = 0

        with self.assertRaisesRegex(AttributeError, u"\\blines\\b"):
            r._check()
