package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.datatype.DataType
import io.kaitai.struct.exprlang.Ast
import io.kaitai.struct.languages.RubyCompiler
import io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import io.kaitai.struct.translators.RubyTranslator
import io.kaitai.struct.{ClassTypeProvider, Utils}

class RubySG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val translator = new RubyTranslator(provider)
  val className = Utils.upperCamelCase(spec.id) // FIXME: proper type-to-class method

  override def fileName(name: String): String = s"${name}_spec.rb"

  override def indentStr: String = "  "

  override def header(): Unit = {
    out.puts(s"require '${spec.id}'")
    out.puts
    out.puts(s"RSpec.describe $className do")
    out.inc
    out.puts(s"it 'parses test properly' do")
    out.inc
    out.puts(s"r = $className.from_file('src/${spec.data}')")
    out.puts
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
