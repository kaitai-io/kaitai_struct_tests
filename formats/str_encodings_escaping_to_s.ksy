# This test checks whether the encoding string passed as an argument to the
# `.to_s(encoding)` method is correctly escaped in the generated code (this
# follows up on the concern that I raised in
# https://github.com/kaitai-io/kaitai_struct_compiler/commit/6500c68cbc6cf13f29a2e002acbccbb9b6fbba16#r122025444).
# It does this by attempting to use encodings that contain characters which need
# to be escaped in many languages (backslash `\`, single quote `'`, double quote
# `"`), but only in such a combination as to avoid leaving from the string
# literal in target languages where KSC doesn't perform escaping properly
# (because that would lead to syntax errors in the generated code, which are a
# bit ugly in CI at the moment, as it usually just displays the status `unknown`
# without any details).
#
# We distinguish between languages with proper escaping and the ones without it
# by inspecting each "unknown encoding" error. If it exactly reproduces the
# encoding name that we passed to the `.to_s(encoding)` method, we know the
# encoding was correctly escaped, otherwise it wasn't and the test should fail.
meta:
  id: str_encodings_escaping_to_s
  endian: le
seq:
  - id: len_of_1
    type: u2
  - id: str1_raw
    size: len_of_1
  - id: len_of_2
    type: u2
  - id: str2_raw
    size: len_of_2
  - id: len_of_3
    type: u2
  - id: str3_raw
    size: len_of_3
  - id: len_of_4
    type: u2
  - id: str4_raw
    size: len_of_4
instances:
  # Note that single-quoted strings in the KS expression language treat every
  # character between the enclosing ' characters literally (so '\\' are two
  # literal backslashes, unlike "\\", which is only one backslash).
  str1:
    value: |
      str1_raw.to_s('ASCII\\x')
  str2:
    value: |
      str2_raw.to_s("UTF-8\\'x")
  str3:
    value: |
      str3_raw.to_s('SJIS\"x')
  str4:
    value: |
      str4_raw.to_s('IBM437\nx')
