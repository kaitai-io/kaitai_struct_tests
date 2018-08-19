package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.ClassTypeProvider
import io.kaitai.struct.datatype.DataType
import io.kaitai.struct.exprlang.Ast
import io.kaitai.struct.languages.PythonCompiler
import io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import io.kaitai.struct.translators.PythonTranslator

class PythonSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  importList.add("import unittest")

  val translator = new PythonTranslator(provider, importList)
  val className = PythonCompiler.type2class(spec.id)

  override def fileName(name: String): String = s"test_$name.py"

  override def indentStr: String = "    "

  override def header(): Unit = {
    out.puts
    out.puts(s"from ${spec.id} import $className")
    out.puts
    out.puts(s"class Test$className(unittest.TestCase):")
    out.inc
    out.puts(s"def test_${spec.id}(self):")
    out.inc
    out.puts(s"with $className.from_file('src/${spec.data}') as r:")
    out.inc
  }

  override def footer(): Unit = {}

  override def simpleAssert(check: TestAssert): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translateExp(check.expected)
    out.puts(s"self.assertEqual($actStr, $expStr)")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"self.assertIsNone($actStr)")
  }

  override def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[Ast.expr]): Unit =
    simpleAssert(check)

  override def noAsserts() =
    out.puts("pass")

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace("self." + Main.INIT_OBJ_NAME, "r")

  def translateExp(x: Ast.expr) =
    translator.translate(x).replace("self._root", className)

  override def results: String =
    "# " + AUTOGEN_COMMENT + "\n\n" + super.results
}
