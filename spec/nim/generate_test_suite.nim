import os, strutils, sequtils, macros

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
  names.add("Nim: " & name[1..^5].split("_").mapIt(capitalizeAscii(it)).join)
  tests.add(readFile("tests" / name))
  imports.add(".." / ".." / "compiled" / "nim" / name[1..^5])

addCode(code):
  import unittest, "../../../runtime/nim/kaitai"

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

code.add("\nsuite \"Kaitai Struct Compiler Test Suite\":\n")

for (name, test) in zip(names, tests):
  code.addTest(name, test)

if ci:
  addCode(code):
    close(outputFormatter)

writeFile("test.nim", strip(code))
