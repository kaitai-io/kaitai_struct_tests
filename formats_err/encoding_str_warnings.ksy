# encoding_str_warnings.ksy: /seq/1/encoding:
# 	warning: use canonical encoding name `UTF-8` instead of `utF-8` (see https://doc.kaitai.io/ksy_style_guide.html#encoding-name)
#
# encoding_str_warnings.ksy: /seq/2/encoding:
# 	warning: use canonical encoding name `ISO-8859-1` instead of `latin1` (see https://doc.kaitai.io/ksy_style_guide.html#encoding-name)
#
# encoding_str_warnings.ksy: /instances/alias/encoding:
# 	warning: use canonical encoding name `ISO-8859-1` instead of `ISo8859-1` (see https://doc.kaitai.io/ksy_style_guide.html#encoding-name)
#
# encoding_str_warnings.ksy: /instances/wrong_case/encoding:
# 	warning: use canonical encoding name `ISO-8859-1` instead of `iSo-8859-1` (see https://doc.kaitai.io/ksy_style_guide.html#encoding-name)
#
meta:
  id: encoding_str_warnings
seq:
  - id: foo
    type: strz
    encoding: UTF-8
  - id: seq_wrong_case
    type: strz
    encoding: utF-8
  - id: seq_alias
    type: str
    size: 3
    encoding: latin1
instances:
  good:
    pos: 123
    type: strz
    encoding: ISO-8859-1
  wrong_case:
    pos: 123
    type: strz
    encoding: iSo-8859-1
  alias:
    pos: 123
    type: strz
    encoding: ISo8859-1
