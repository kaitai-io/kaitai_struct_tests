package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.{ClassTypeProvider, Utils}
import io.kaitai.struct.datatype.DataType
import io.kaitai.struct.exprlang.Ast
import io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import io.kaitai.struct.translators.ConstructTranslator

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
    out.puts(s"r = _schema.parse_file('src/${spec.data}')")
  }

  override def footer(): Unit = {}

  override def simpleAssert(check: TestAssert): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"self.assertEqual($actStr, $expStr)")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"self.assertIsNone($actStr)")
  }

  override def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[Ast.expr]): Unit =
    simpleAssert(check)

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace("this." + Main.INIT_OBJ_NAME, "r")

  override def results: String =
    "# " + AUTOGEN_COMMENT + "\n\n" + super.results
}
