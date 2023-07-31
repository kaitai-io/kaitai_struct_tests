package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.{ClassTypeProvider, Utils}
import _root_.io.kaitai.struct.datatype.DataType
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestEquals, TestSpec}
import _root_.io.kaitai.struct.translators.ConstructTranslator

class ConstructSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  importList.add("import unittest")

  val translator = new ConstructTranslator(provider, importList)

  override def fileName(name: String): String = s"test_$name.py"

  override def indentStr: String = "    "

  override def header(): Unit = {
    out.puts
    out.puts(s"from ${spec.id} import _schema")
    out.puts
    out.puts(s"class Test${Utils.upperCamelCase(spec.id)}(unittest.TestCase):")
    out.inc
    out.puts(s"def test_${spec.id}(self):")
    out.inc
  }

  override def runParse(): Unit =
    out.puts(s"r = _schema.parse_file('src/${spec.data}')")

  override def footer(): Unit = {}

  override def simpleEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"self.assertEqual($actStr, $expStr)")
  }

  override def floatEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"self.assertAlmostEqual($actStr, $expStr, 6)")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"self.assertIsNone($actStr)")
  }

  override def trueArrayEquality(check: TestEquals, elType: DataType, elts: Seq[Ast.expr]): Unit =
    simpleEquality(check)

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace("this." + Main.INIT_OBJ_NAME, "r")

  override def results: String =
    "# " + AUTOGEN_COMMENT + "\n\n" + super.results
}
