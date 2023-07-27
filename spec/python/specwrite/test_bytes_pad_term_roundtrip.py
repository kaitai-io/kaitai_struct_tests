import unittest
from specwrite.common_spec import CommonSpec

from testwrite.bytes_pad_term_roundtrip import BytesPadTermRoundtrip

class TestBytesPadTermRoundtrip(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestBytesPadTermRoundtrip, self).__init__(*args, **kwargs)
        self.struct_class = BytesPadTermRoundtrip
        self.src_filename = 'src/str_pad_term.bin'

    def test_bytes_pad_term_roundtrip(self):
        # NOTE: here it makes sense to prefer a manual test over the automatic
        # testReadWriteRoundtrip, because the roundtrip can't always recognize whether the
        # `pad-right` is correct (i.e. whether the serialized field is padded correctly)

        r = BytesPadTermRoundtrip()

        r.str_pad = b"str1"
        r.str_term = b"str2foo"
        r.str_term_and_pad = b"str+++3bar+++"
        r.str_term_include = b"str4baz@"

        self.assert_equal_to_full_file(r, 'src/str_pad_term.bin')
