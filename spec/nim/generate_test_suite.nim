import os, strutils, sequtils, macros

let excluded = @[
  "imports0",
  "bits_simple",
  "cast_nested",
  "cast_to_imported",
  "cast_to_top",
  "str_pad_term",
  "nested_same_name2",
  "default_endian_expr_is_be",
  "expr_io_pos",
  "process_coerce_usertype2",
  "switch_manual_str",
  "nav_parent",
  "nav_parent2",
  "term_bytes",
  "enum_deep_literals",
  "expr_1",
  "expr_2",
  "expr_3",
  "expr_array",
  "fixed_contents",
  "fixed_struct",
  "if_struct",
  "if_instances",
  "if_values",
  "index_sizes",
  "index_to_param_until",
  "index_to_param_eos",
  "index_to_param_expr",
  "instance_io_user",
  "instance_std_array",
  "instance_user_array",
  "integers",
  "io_local_var",
  "multiple_use",
  "nav_parent3",
  "nav_parent_false",
  "nav_parent_switch",
  "nav_parent_vs_value_inst",
  "nav_parent_override",
  "nav_root",
  "nested_same_name",
  "nested_types",
  "nested_types2",
  "nested_types3",
  "non_standard",
  "params_call_short",
  "params_pass_usertype",
  "process_coerce_bytes",
  "process_coerce_usertype1",
  "process_coerce_switch",
  "process_to_user",
  "position_abs",
  "position_to_end",
  "str_eos",
  "str_encodings",
  "str_encodings_default",
  "str_pad_term_empty",
  "switch_bytearray",
  "switch_integers",
  "switch_integers2",
  "switch_manual_int",
  "switch_manual_int_else",
  "switch_manual_str_else",
  "switch_manual_int_size_else",
  "recursive_one",
  "repeat_n_strz",
  "repeat_n_strz_double",
  "repeat_until_s4",
  "type_ternary",
  "term_strz",
  "valid_short",
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
  proc `==`[T](x: Option[T], y: T): bool =
    get(x) == y

code.add("\nsuite \"Kaitai Struct Compiler Test Suite\":\n")

for (name, test) in zip(names, tests):
  code.addTest(name, test)

if ci:
  addCode(code):
    close(outputFormatter)

writeFile("test.nim", strip(code))
