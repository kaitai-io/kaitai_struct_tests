package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.ClassTypeProvider
import io.kaitai.struct.datatype.DataType
import io.kaitai.struct.datatype.DataType.BytesType
import io.kaitai.struct.exprlang.Ast
import io.kaitai.struct.languages.JavaScriptCompiler
import io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import io.kaitai.struct.translators.JavaScriptTranslator

class JavaScriptSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val className = JavaScriptCompiler.type2class(spec.id)
  val translator = new JavaScriptTranslator(provider)

  importList.add("assert")
  importList.add("testHelper")

  override def fileName(name: String): String = s"test_${spec.id}.js"

  override def header(): Unit = {
    out.puts(s"testHelper('$className', 'src/${spec.data}', function(r) {")
    out.inc
  }

  override def footer(): Unit = {
    out.dec
    out.puts("});")
  }

  override def simpleAssert(check: TestAssert): Unit = {
    val actType = translator.detectType(check.actual)
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    actType match {
      case _: BytesType =>
        importList.add("hexString")
        out.puts(s"assert.strictEqual(hexString($actStr), hexString($expStr));")
      case _ =>
        out.puts(s"assert.strictEqual($actStr, $expStr);")
    }
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"assert.strictEqual($actStr, undefined);")
  }

  override def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[Ast.expr]): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"assert.deepStrictEqual($actStr, $expStr);")
  }

  override def indentStr: String = "  "

  override def results: String = {
    "// " + AUTOGEN_COMMENT + "\n\n" +
      importList.toList.map((x) => s"var $x = require('$x');").mkString("", "\n", "\n") + "\n" +
      out.result
  }

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace(s"this.${Main.INIT_OBJ_NAME}", "r")
}
