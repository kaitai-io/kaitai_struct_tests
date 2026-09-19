package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.ClassTypeProvider
import io.kaitai.struct.datatype.EndOfStreamError
import io.kaitai.struct.languages.GoCompiler
import io.kaitai.struct.testtranslator.TestSpec

/** Native serialization tests, using the same KST selection as Java/Python. */
class GoWriteSG(spec: TestSpec, provider: ClassTypeProvider) extends SpecGenerator {
  val className = GoCompiler.types2class(List(spec.id))

  override def fileName(name: String): String = s"specwrite/${name}_test.go"
  override def indentStr: String = "\t"

  override def run(): Unit = {
    importList.add("\"testing\"")
    importList.add("\"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai\"")
    importList.add(". \"test_formats_write\"")
    out.puts(s"func Test$className(t *testing.T) {")
    out.inc
    out.puts("parse := func(s *kaitai.Stream) (writeStruct, error) {")
    out.inc
    out.puts(s"var r $className")
    out.puts("err := r.Read(s, &r, &r)")
    out.puts("return &r, err")
    out.dec
    out.puts("}")
    spec.exception match {
      case None =>
        out.puts(s"assertRoundTrip(t, \"../../../src/${spec.data}\", parse)")
      case Some(expected) =>
        out.puts(s"_, err := parseFile(\"../../../src/${spec.data}\", parse)")
        expected.exception match {
          case EndOfStreamError =>
            importList.add("\"io\"")
            out.puts("assertErrorIs(t, err, io.ErrUnexpectedEOF)")
          case exception =>
            out.puts(s"var wantErr ${GoCompiler.ksErrorName(exception)}")
            out.puts("assertErrorAs(t, err, &wantErr)")
        }
        out.puts("if t.Failed() {")
        out.inc
        out.puts("return")
        out.dec
        out.puts("}")
        out.puts("t.Skip(\"cannot use roundtrip because parsing is expected to fail\")")
    }
    out.dec
    out.puts("}")
  }

  override def results: String =
    "// " + AUTOGEN_COMMENT + "\n\npackage specwrite\n\nimport (\n" +
      importList.toList.sortBy(_.split('"')(1)).map(x => "\t" + x).mkString("\n") + "\n)\n\n" + out.result
}
