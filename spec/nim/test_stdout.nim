import os, strutils

let
  fileLines = splitLines(readFile("spec" / "nim" / "tests" / paramStr(1) & ".nim"))
  header = fileLines[0..4].join("\n")
  footer = fileLines[10..^3].join("\n")
writeFile("spec" / "nim" / "tests" / "test.nim", header & footer)