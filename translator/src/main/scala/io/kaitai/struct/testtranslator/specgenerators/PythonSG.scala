package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.{ClassTypeProvider, RuntimeConfig}
import _root_.io.kaitai.struct.datatype.{DataType, KSError}
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.PythonCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestEquals, TestSpec}
import _root_.io.kaitai.struct.translators.PythonTranslator

class PythonSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  importList.add("import unittest")

  val config = RuntimeConfig()
  val translator = new PythonTranslator(provider, importList, config)
  val className = PythonCompiler.type2class(spec.id)

  importList.add(s"from ${spec.id} import $className")
  spec.extraImports.foreach(entry =>
    importList.add(s"from $entry import ${PythonCompiler.type2class(entry)}")
  )

  override def fileName(name: String): String = s"test_$name.py"

  override def indentStr: String = "    "

  override def header(): Unit = {
    out.puts
    out.puts(s"class Test$className(unittest.TestCase):")
    out.inc
    out.puts(s"def test_${spec.id}(self):")
    out.inc
  }

  override def runParse(): Unit = {
    out.puts(s"with $className.from_file('src/${spec.data}') as r:")
    out.inc
  }

  override def runParseExpectError(exception: KSError): Unit = {
    importList.add("import kaitaistruct")
    out.puts(s"with self.assertRaises(${PythonCompiler.ksErrorName(exception)}):")
    out.inc
    runParse()
    out.puts("pass")
  }

  override def footer(): Unit = {}

  override def simpleEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translateExp(check.expected)
    out.puts(s"self.assertEqual($actStr, $expStr)")
  }

  override def floatEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translateExp(check.expected)
    out.puts(s"self.assertAlmostEqual($actStr, $expStr, 6)")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"self.assertIsNone($actStr)")
  }

  override def trueArrayEquality(check: TestEquals, elType: DataType, elts: Seq[Ast.expr]): Unit =
    simpleEquality(check)

  override def testException(actual: Ast.expr, exception: KSError): Unit = {
    importList.add("import kaitaistruct")
    out.puts(s"with self.assertRaises(${PythonCompiler.ksErrorName(exception)}):")
    out.inc
    out.puts(translateAct(actual))
    out.dec
  }

  override def noAsserts() =
    out.puts("pass")

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace("self." + Main.INIT_OBJ_NAME, "r")

  def translateExp(x: Ast.expr) =
    translator.translate(x).replace("self._root", className)

  override def results: String =
    "# " + AUTOGEN_COMMENT + "\n\n" + super.results
}
