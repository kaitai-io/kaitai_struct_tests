import os, strutils, sequtils, macros

let excluded = @[
  "bcd_user_type_be",
  "bcd_user_type_le",
  "bits_byte_aligned",
  "bits_simple",
  "buffered_struct",
  "bytes_pad_term",
  "cast_nested",
  "cast_to_imported",
  "cast_to_top",
  "default_big_endian",
  "default_endian_expr_inherited",
  "default_endian_expr_is_be",
  "default_endian_expr_is_le",
  "default_endian_mod",
  "docstrings_docref_multi",
  "docstrings_docref",
  "docstrings",
  "enum_deep_literals",
  "expr_0",
  "expr_1",
  "expr_2",
  "expr_3",
  "expr_array",
  "expr_bytes_cmp",
  "expr_io_pos",
  "expr_mod",
  "expr_sizeof_type_0",
  "expr_sizeof_type_1",
  "expr_sizeof_value_0",
  "expr_sizeof_value_sized",
  "fixed_contents",
  "fixed_struct",
  "floating_points",
  "float_to_i",
  "hello_world",
  "if_instances",
  "if_struct",
  "if_values",
  "imports0",
  "index_sizes",
  "index_to_param_eos",
  "index_to_param_expr",
  "index_to_param_until",
  "instance_io_user",
  "instance_std_array",
  "instance_std",
  "instance_user_array",
  "integers",
  "io_local_var",
  "js_signed_right_shift",
  "meta_tags",
  "meta_xref",
  "multiple_use",
  "nav_parent2",
  "nav_parent3",
  "nav_parent_false2",
  "nav_parent_false",
  "nav_parent",
  "nav_parent_override",
  "nav_parent_switch",
  "nav_parent_vs_value_inst",
  "nav_root",
  "nested_same_name2",
  "nested_same_name",
  "nested_types2",
  "nested_types3",
  "nested_types",
  "non_standard",
  "params_call_short",
  "params_pass_usertype",
  "position_abs",
  "position_in_seq",
  "position_to_end",
  "process_coerce_bytes",
  "process_coerce_switch",
  "process_coerce_usertype1",
  "process_coerce_usertype2",
  "process_custom",
  "process_rotate",
  "process_to_user",
  "process_xor4_const",
  "process_xor4_value",
  "process_xor_const",
  "process_xor_value",
  "recursive_one",
  "repeat_eos_bit",
  "repeat_eos_struct",
  "repeat_eos_u4",
  "repeat_n_struct",
  "repeat_n_strz_double",
  "repeat_n_strz",
  "repeat_until_complex",
  "repeat_until_s4",
  "repeat_until_sized",
  "str_encodings_default",
  "str_encodings",
  "str_eos",
  "str_literals2",
  "str_pad_term_empty",
  "str_pad_term",
  "switch_bytearray",
  "switch_integers2",
  "switch_integers",
  "switch_manual_int_else",
  "switch_manual_int",
  "switch_manual_int_size_else",
  "switch_manual_str_else",
  "switch_manual_str",
  "term_bytes",
  "term_strz",
  "type_int_unary_op",
  "type_ternary",
  "user_type",
  "valid_short",
  "zlib_with_header_78",
]

macro addCode(where, what: untyped): untyped =
  result = newCall("add", where, toStrLit(what))

template addImport(where, what: string) =
  where.add("import " & what & "\n")

template addTest(where: untyped; name, code: string) =
  where.add("  test \"" & name & "\":\n")
  let linesOfCode = splitLines(code)
  for l in linesOfCode:
    where.add("    " & l & "\n")

if paramCount() notin {0,1}:
  echo "Wrong number of arguments"
  quit QuitFailure

var ci: bool

if paramCount() == 1:
  if paramStr(1) == "ci":
    ci = true
  else:
    echo "The only valid argument is 'ci'"
    quit QuitFailure

setCurrentDir("spec/nim")

var
  code: string
  names = newSeq[string]()
  tests = newSeq[string]()
  imports = newSeq[string]()

for _, name in walkDir("tests", true):
  let file = ".." / ".." / "compiled" / "nim" / name[1..^1]
  if name[1..^5] notin excluded and fileExists(file):
    imports.add(".." / ".." / "compiled" / "nim" / name[1..^5])
    names.add("Nim: " & name[1..^5].split("_").mapIt(capitalizeAscii(it)).join)
    tests.add(readFile("tests" / name))

addCode(code):
  import os, options, unittest, "../../../runtime/nim/kaitai"

if ci:
  addCode(code):
    import streams
    var
      stream = newFileStream("test_out/nim/report.xml", fmWrite)
      outputFormatter = newJUnitOutputFormatter(stream)
    addOutputFormatter(outputFormatter)

code.add("\n\n")

for i in imports:
  code.addImport(i)

addCode(code):
  proc `==`[T: SomeInteger](x: seq[T]; y: seq[int]): bool =
    result = true
    for i in 0 ..< x.len:
      if int(x[i]) != y[i]:
        return false

code.add("\nsuite \"Kaitai Struct Compiler Test Suite\":\n")

for (name, test) in zip(names, tests):
  code.addTest(name, test)

if ci:
  addCode(code):
    close(outputFormatter)

writeFile("test.nim", strip(code))
