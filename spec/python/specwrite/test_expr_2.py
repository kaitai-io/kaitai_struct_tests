# -*- coding: utf-8 -*-
import io
import unittest
from kaitaistruct import KaitaiStream
from specwrite.common_spec import CommonSpec

from testwrite.expr_2 import Expr2

class TestExpr2(CommonSpec.Base):
    def __init__(self, *args, **kwargs):
        super(TestExpr2, self).__init__(*args, **kwargs)
        self.struct_class = Expr2
        self.src_filename = 'src/str_encodings.bin'

    def test_edit(self):
        r = Expr2.from_file(self.src_filename)
        r._read()

        old_len_mod = r.str2.len_mod
        old_str2_rest_avg = r.str2.rest.avg

        r.str2.str = u"Kaitai Struct カ"
        r.str2._invalidate_len_mod()

        str2_size = len(r.str2.str.encode(u"UTF-8"))
        r.str2.len_orig = str2_size + 3
        self.assertNotEqual(r.str2.len_mod, old_len_mod)
        self.assertEqual(r.str2.len_mod, str2_size)

        r.str2.rest.byte0 = 0xfa
        r.str2.rest._invalidate_avg()
        r.str2.rest.byte1 = 0x44
        r.str2.rest.byte2 = 0x88
        self.assertNotEqual(r.str2.rest.avg, old_str2_rest_avg)

        # see <CommonSpec.Base>.test_read_write_roundtrip
        orig_dump = CommonSpec.Base.dump_struct(r)

        new_io = KaitaiStream(io.BytesIO(bytearray(
            2 +  # str1.len_orig
            r.str1.len_mod +  # str1.str
            3 +  # str1.rest
            2 +  # str2.len_orig
            r.str2.len_mod +  # str2.str
            3  # str2.rest
        )))
        r._write(new_io)
        new_io.seek(0)

        new_r = Expr2(new_io)
        new_r._read()

        new_dump = CommonSpec.Base.dump_struct(new_r)

        self.assertEqual(new_dump, orig_dump)
