package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.{ClassTypeProvider, Utils}
import _root_.io.kaitai.struct.exprlang.Ast.expr
import _root_.io.kaitai.struct.datatype.DataType
import _root_.io.kaitai.struct.translators.{NimTranslator, TypeDetector}
import _root_.io.kaitai.struct.Utils
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}

class NimSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val className = Utils.upperCamelCase(spec.id)
  val translator = new NimTranslator(provider, importList)

  // Members declared in io.kaitai.struct.testtranslator.specgenerators.BaseGenerator
  override def fileName(name: String): String = s"t${spec.id}.nim"
  override def header(): Unit = {
    out.puts(s"let r = ${className}.fromFile" + "(\"src/" + spec.data + "\")")
  }
  override def footer(): Unit = {
  }
  override def nullAssert(actual: expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"doAssert($actStr == none(typeof($actStr)))")
  }
  override def simpleAssert(check: TestAssert): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    val td = new TypeDetector(provider)
    val t = _root_.io.kaitai.struct.NimClassCompiler.ksToNim(td.detectType(check.actual))
    out.puts(s"doAssert($actStr == $t($expStr))")
  }
  override def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[expr]): Unit = {
    simpleAssert(check)
  }
  override def runParse(): Unit = {
    importList.add("os, streams, options, sequtils, ../testhelpers")
    importList.add("../../../compiled/nim/" + spec.id)
  }

  // Members declared in io.kaitai.struct.testtranslator.specgenerators.SpecGenerator
  override def indentStr: String = "  "

  override def results: String = {
    "# " + AUTOGEN_COMMENT + "\n\n" +
      importList.toList.map((x) => s"import $x").mkString("", "\n", "\n") + "\n" + out.result
  }

  // Memebers declared here
  def translateAct(x: expr) =
    translator.translate(x).replace(Utils.lowerCamelCase(Main.INIT_OBJ_NAME), "r")
}
