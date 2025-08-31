package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.{ClassTypeProvider, RuntimeConfig}
import _root_.io.kaitai.struct.datatype.{DataType, KSError}
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.PythonCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestEquals, TestSpec, ExpectedException}
import _root_.io.kaitai.struct.translators.PythonTranslator

class PythonSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  importList.add("import unittest")

  val config = RuntimeConfig(pythonPackage = "testformats")
  val translator = new PythonTranslator(provider, importList, config)
  val className = PythonCompiler.type2class(spec.id)

  importList.add(s"from testformats.${spec.id} import $className")
  // NOTE: some other languages use `spec.extraImports` here, but since we pass
  // `importList` to PythonTranslator above, which will insert necessary imports
  // when translating external enum literals itself, we don't need to use it
  // here. It would only generate redundant imports.

  override def fileName(name: String): String = s"spec/test_$name.py"

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

  override def runParseExpectError(expException: ExpectedException): Unit = {
    val exception = expException.exception
    importList.add("import kaitaistruct")
    expException.message match {
      case Some(_) =>
        out.puts(s"with self.assertRaises(${PythonCompiler.ksErrorName(exception)}) as cm:")
      case None =>
        out.puts(s"with self.assertRaises(${PythonCompiler.ksErrorName(exception)}):")
    }
    out.inc
    runParse()
    out.puts("pass")
    out.dec
    out.dec
    expException.message match {
      case Some(msg) =>
        val expStr = translator.translate(Ast.expr.Str(msg))
        out.puts(s"self.assertEqual(str(cm.exception), $expStr)")
      case None =>
    }
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
