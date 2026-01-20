package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.datatype.{DataType, EndOfStreamError, KSError}
import _root_.io.kaitai.struct.datatype.DataType._
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.GoCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestEquals, TestSpec, ExpectedException}
import _root_.io.kaitai.struct.translators.{GoTranslator, TypeProvider}
import _root_.io.kaitai.struct.{ClassTypeProvider, ImportList, RuntimeConfig, StringLanguageOutputWriter, Utils}

class GoSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  /**
    * Special wrapper around StringLanguageOutputWriter that catches all attempts
    * to access "this.INIT_OBJ_NAME" and replaces it with "r."
    */
  class GoOutputWriter(indentStr: String) extends StringLanguageOutputWriter(indentStr) {
    override def puts(s: String): Unit = {
      super.puts(s.replace(REPLACER, "r."))
    }
  }

  /**
    * Special wrapper around translator that catches all attempts to write error
    * check and turns it into assertion.
    */
  class GoTestTranslator(
    out: StringLanguageOutputWriter,
    provider: TypeProvider,
    importList: ImportList,
  ) extends GoTranslator(out, provider, importList) {
    var doErrCheck = true

    override def outAddErrCheck(): Unit = {
      if (doErrCheck) {
        out.puts("if err != nil {")
        out.inc
        out.puts("t.Fatal(err)")
        out.dec
        out.puts("}")
      }
    }
  }

  override val out = new GoOutputWriter(indentStr)
  val compiler = new GoCompiler(provider, RuntimeConfig())
  val className = GoCompiler.types2class(List(spec.id))
  val translator = new GoTestTranslator(out, provider, importList)

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
    checkErr(exception)
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

  override def testException(actual: Ast.expr, exception: KSError): Unit = {
    // We need a scope otherwise we got redeclaration error from Go in case of
    // several assertions, because we use the same name for expected exception
    out.puts("{")
    out.inc

    // We do not want error check because we expect an error
    translator.doErrCheck = false
    val actStr = translateAct(actual)
    translator.doErrCheck = true

    checkErr(exception)

    // translateAct generates unused variable which not allowed in Go,
    // so we use it by checking its value
    translator.detectType(actual) match {
      case _: FloatType => out.puts(s"assert.InDelta(t, 0, $actStr, $FLOAT_DELTA)")
      case _: NumericType => out.puts(s"assert.EqualValues(t, 0, $actStr)")
      case _: BooleanType => out.puts(s"assert.EqualValues(t, false, $actStr)")
      case _: StrType => out.puts(s"assert.EqualValues(t, \"\", $actStr)")
      case _ => out.puts(s"assert.Nil(t, $actStr)")
    }

    out.dec
    out.puts("}")
  }

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

  /** Generates code to check returned Go error to match of specified `exception`. */
  def checkErr(exception: KSError): Unit = {
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
}
