package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.datatype.{DataType, EndOfStreamError, KSError}
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.GoCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestEquals, TestSpec, ExpectedException}
import _root_.io.kaitai.struct.translators.GoTranslator
import _root_.io.kaitai.struct.{ClassTypeProvider, RuntimeConfig, StringLanguageOutputWriter, Utils}

class GoSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  /**
    * Special wrapper around StringLanguageOutputWriter that catches all attempts
    * to access "this.INIT_OBJ_NAME" and replaces it with "r."
    */
  class GoOutputWriter(out: StringLanguageOutputWriter) extends StringLanguageOutputWriter(indentStr) {
    override def inc: Unit = out.inc
    override def dec: Unit = out.dec
    override def indentNow: String = out.indentNow

    override def add(other: StringLanguageOutputWriter): Unit = out.add(other)
    override def puts(s: String): Unit = {
      val mangled = s.replace(REPLACER, "r.").replaceAll("return err$", "t.Fatal(err)")
      out.puts(mangled)
    }
    override def puts = out.puts
    override def close = out.close
    override def putsLines(prefix: String, lines: String, hanging: String): Unit =
      out.putsLines(prefix, lines, hanging)

    override def result: String = out.result
  }

  val compiler = new GoCompiler(provider, RuntimeConfig())
  val className = GoCompiler.types2class(List(spec.id))
  val translator = new GoTranslator(new GoOutputWriter(out), provider, importList)

  override def fileName(name: String): String = s"${name}_test.go"

  importList.add("\"runtime/debug\"")
  importList.add("\"os\"")
  importList.add("\"testing\"")
  importList.add("\"github.com/kaitai-io/kaitai_struct_go_runtime/kaitai\"")
  importList.add(". \"test_formats\"")

  override def header() = {
    out.puts(s"func Test$className(t *testing.T) {")
    out.inc
    out.puts("defer func() {")
    out.inc
    out.puts("if r := recover(); r != nil {")
    out.inc
    out.puts("debug.PrintStack()")
    out.puts("t.Fatal(\"unexpected panic:\", r)")
    out.dec
    out.puts("}")
    out.dec
    out.puts("}()")
    out.puts("f, err := os.Open(\"../../src/" + spec.data + "\")")
    fatalCheck()
    out.puts("s := kaitai.NewStream(f)")
    out.puts(s"var r $className")
  }

  override def runParse(): Unit = {
    out.puts("err = r.Read(s, &r, &r)")
    fatalCheck()
  }

  override def runParseExpectError(expException: ExpectedException): Unit = {
    val exception = expException.exception
    out.puts("err = r.Read(s, &r, &r)")
    importList.add("\"github.com/stretchr/testify/assert\"")
    out.puts("assert.Error(t, err)")
    exception match {
      case EndOfStreamError =>
        importList.add("\"io\"")
        out.puts("assert.ErrorIs(t, err, io.ErrUnexpectedEOF)")
      case _ =>
        val errorName = GoCompiler.ksErrorName(exception)
        out.puts(s"var wantErr ${errorName}")
        out.puts("assert.ErrorAs(t, err, &wantErr)")
    }
  }

  override def footer() = {
    out.dec
    out.puts("}")
  }

  def simpleEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    importList.add("\"github.com/stretchr/testify/assert\"")
    out.puts(s"assert.EqualValues(t, $expStr, $actStr)")
  }

  override def floatEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    importList.add("\"github.com/stretchr/testify/assert\"")
    out.puts(s"assert.InDelta(t, $expStr, $actStr, $FLOAT_DELTA)")
  }

  def nullAssert(actual: Ast.expr): Unit = {
    importList.add("\"github.com/stretchr/testify/assert\"")
    val actStr = translateAct(actual)
    out.puts(s"assert.Nil(t, $actStr)")
  }

  def trueArrayEquality(check: TestEquals, elType: DataType, elts: Seq[Ast.expr]): Unit =
    simpleEquality(check)

  override def indentStr: String = "\t"

  override def results: String = {
    "// " + AUTOGEN_COMMENT + "\n\n" +
    "package spec\n\nimport (\n" +
      importList.toList.map((x) => "\t" + x).mkString("\n") +
      "\n)\n\n" +
      out.result
  }

  def fatalCheck() = {
    out.puts("if err != nil {")
    out.inc
    out.puts("t.Fatal(err)")
    out.dec
    out.puts("}")
  }

  lazy val REPLACER: String = "this." + Utils.upperCamelCase(Main.INIT_OBJ_NAME) + "."

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace(REPLACER, "r.")
}
