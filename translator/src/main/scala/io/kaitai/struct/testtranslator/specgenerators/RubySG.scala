package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.datatype.{DataType, KSError}
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import _root_.io.kaitai.struct.translators.RubyTranslator
import _root_.io.kaitai.struct.{ClassTypeProvider, Utils}
import io.kaitai.struct.languages.RubyCompiler

class RubySG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val translator = new RubyTranslator(provider)
  val className = Utils.upperCamelCase(spec.id) // FIXME: proper type-to-class method

  override def fileName(name: String): String = s"${name}_spec.rb"

  override def indentStr: String = "  "

  override def header(): Unit = {
    out.puts(s"RSpec.describe '$className' do")
    out.inc

    out.puts(s"it 'parses test properly' do")
    out.inc
    out.puts(s"require '${spec.id}'")
    spec.extraImports.foreach(fn => out.puts(s"require '$fn'"))
  }

  override def runParse(): Unit = {
    out.puts(s"r = $className.from_file('src/${spec.data}')")
  }

  override def runParseExpectError(exception: KSError): Unit = {
    out.puts("expect {")
    out.inc
    runParse()
    out.dec
    out.puts(s"}.to raise_error(${RubyCompiler.ksErrorName(exception)})")
  }

  override def footer(): Unit = {
    out.dec
    out.puts("end")
    out.dec
    out.puts("end")
  }

  override def simpleAssert(check: TestAssert): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"expect($actStr).to eq $expStr")
  }

  override def floatAssert(check: TestAssert): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"expect($actStr).to be_within($FLOAT_DELTA).of $expStr")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"expect($actStr).to be_nil")
  }

  override def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[Ast.expr]): Unit =
    simpleAssert(check)

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace(Main.INIT_OBJ_NAME, "r")

  override def results: String =
    "# " + AUTOGEN_COMMENT + "\n" + super.results
}
