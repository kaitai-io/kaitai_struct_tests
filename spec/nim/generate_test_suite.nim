import os, strutils, sequtils, macros

let excluded = @[
  "imports0",
  "cast_nested",
  "cast_to_imported",
  "str_pad_term",
  "nested_same_name2",
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
  "index_sizes",
  "index_to_param_until",
  "index_to_param_eos",
  "index_to_param_expr",
  "instance_io_user",
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
  if name[1..^5] notin excluded:
    names.add("Nim: " & name[1..^5].split("_").mapIt(capitalizeAscii(it)).join)
    tests.add(readFile("tests" / name))
    if fileExists(".." / ".." / "compiled" / "nim" / name[1..^1]):
      imports.add(".." / ".." / "compiled" / "nim" / name[1..^5])

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
  converter dropOption[T](x: Option[T]): T = get(x)
  proc `==`[T: SomeSignedInt](x: uint64, y: T): bool =
    x == uint64(y)
  proc `==`[T: SomeInteger](x: seq[T]; y: seq[int]): bool =
    result = true
    for i in 0 .. x.len:
      if int(x[i]) != y[i]:
        return false
  template `[]`[T](x: Option[T], idx: untyped): untyped = get(x)[idx]
  {.experimental: "dotOperators".}
  template `.`[T](x: Option[T], field: untyped): untyped = get(x).field

code.add("\nsuite \"Kaitai Struct Compiler Test Suite\":\n")

for (name, test) in zip(names, tests):
  code.addTest(name, test)

if ci:
  addCode(code):
    close(outputFormatter)

writeFile("test.nim", strip(code))
