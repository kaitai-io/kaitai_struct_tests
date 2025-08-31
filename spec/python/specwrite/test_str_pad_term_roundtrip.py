import unittest
from specwrite.common_spec import CommonSpec

from testwrite.str_pad_term_roundtrip import StrPadTermRoundtrip

class TestStrPadTermRoundtrip(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestStrPadTermRoundtrip, self).__init__(*args, **kwargs)
        self.struct_class = StrPadTermRoundtrip
        self.src_filename = 'src/str_pad_term.bin'

    def test_str_pad_term_roundtrip(self):
        # NOTE: here it makes sense to prefer a manual test over the automatic
        # testReadWriteRoundtrip, because the roundtrip can't always recognize whether the
        # `pad-right` is correct (i.e. whether the serialized field is padded correctly)

        r = StrPadTermRoundtrip()

        r.str_pad = "str1"
        r.str_term = "str2foo"
        r.str_term_and_pad = "str+++3bar+++"
        r.str_term_include = "str4baz@"

        self.assert_equal_to_full_file(r, 'src/str_pad_term.bin')
