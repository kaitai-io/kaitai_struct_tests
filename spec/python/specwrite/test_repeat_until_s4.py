import unittest
from kaitaistruct import ConsistencyError
from specwrite.common_spec import CommonSpec

from testwrite.repeat_until_s4 import RepeatUntilS4

class TestRepeatUntilS4(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestRepeatUntilS4, self).__init__(*args, **kwargs)
        self.struct_class = RepeatUntilS4
        self.src_filename = 'src/repeat_until_s4.bin'

    def test_check_bad_no_entries(self):
        r = RepeatUntilS4()
        r.entries = [0]
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: entries,"):
            r._check()

    def test_check_good_one_entry(self):
        r = RepeatUntilS4()
        r.entries = [-1]
        r.afterall = ""
        r._check()

    def test_check_bad_early_until_entry(self):
        r = RepeatUntilS4()
        r.entries = [-3, 275000, -1, 0, -1]
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: entries,"):
            r._check()

    def test_check_bad_no_until_entry(self):
        r = RepeatUntilS4()
        r.entries = [-3, 275000, -2, 0]
        with self.assertRaisesRegex(ConsistencyError, u"^Check failed: entries,"):
            r._check()

    def test_check_good_until_entry(self):
        r = RepeatUntilS4()
        r.entries = [-3, 275000, -2, 0, -1]
        r.afterall = ""
        r._check()
