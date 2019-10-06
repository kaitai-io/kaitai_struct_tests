package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.{ClassTypeProvider, Utils}
import _root_.io.kaitai.struct.exprlang.Ast.expr
import _root_.io.kaitai.struct.datatype.DataType
import _root_.io.kaitai.struct.languages.NimCompiler
import _root_.io.kaitai.struct.translators.NimTranslator
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}

class NimSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val className = NimCompiler.type2class(spec.id)
  val translator = new NimTranslator(provider, importList)

  // Members declared in io.kaitai.struct.testtranslator.specgenerators.BaseGenerator
  override def fileName(name: String): String = s"src/io/kaitai/struct/spec/t${spec.id}.nim"
  override def footer(): Unit = ()
  override def header(): Unit = ()
  override def nullAssert(actual: expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"check($actStr == none(typeof($actStr)))")
  }
  override def simpleAssert(check: TestAssert): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"check($actStr == $expStr)")
  }
  override def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[expr]): Unit = {
    simpleAssert(check)
  }
  override def runParse(): Unit = {
    out.puts(s"let r = ${className}.fromFile" + "(\"src\" / \"" + spec.data + "\")")
  }

  // Members declared in io.kaitai.struct.testtranslator.specgenerators.SpecGenerator
  override def indentStr: String = "  "

  override def results: String = {
    "# " + AUTOGEN_COMMENT + "\n" +
      importList.toList.map((x) => s"import $x").mkString("", "\n", "\n") +
      out.result
  }

  // Memebers declared here
  def translateAct(x: expr) =
    translator.translate(x).replace(Utils.lowerCamelCase(Main.INIT_OBJ_NAME), "r")
}
