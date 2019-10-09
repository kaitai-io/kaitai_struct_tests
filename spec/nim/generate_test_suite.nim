import os, macros, strutils, sequtils, strformat

let testOutDir = "test_out" / "nim"
var excluded: seq[string]

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

var
  code: string
  names = newSeq[string]()
  tests = newSeq[string]()
  imports = newSeq[string]()

for _, name in walkDir("spec" / "nim" / "tests", true):
  let file = "compiled" / "nim" / name[1..^1]
  if fileExists(file):
    let
      i = ".." / ".." / "compiled" / "nim" / name[1..^5]
      t = readFile("spec" / "nim" / "tests" / name)
    var test = ""
    addCode(test):
      import os, options, unittest, "../../../runtime/nim/kaitai"
    test.addImport(i)
    test.add("\n")
    addCode(test):
      proc `==`[T: SomeInteger](x: seq[T]; y: seq[int]): bool =
        result = true
        for i in 0 ..< x.len:
          if int(x[i]) != y[i]:
            return false
      proc `==`[T](x: Option[T], y: T): bool =
        get(x) == y
    test.add(t)

    let path = testOutDir / "currentlyTesting.nim"
    writeFile(path, test)
    let code = execShellCmd(&"nim c -r --outdir:\"{testOutDir}/bin\" \"{path}\"")
    if code == 0:
      imports.add(i)
      names.add("Nim: " & name[1..^5].split("_").mapIt(capitalizeAscii(it)).join)
      tests.add(t)

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

writeFile("spec" / "nim" / "test.nim", strip(code))
