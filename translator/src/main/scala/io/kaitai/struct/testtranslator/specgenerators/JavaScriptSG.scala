package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.ClassTypeProvider
import _root_.io.kaitai.struct.datatype.{DataType, KSError}
import _root_.io.kaitai.struct.datatype.DataType.BytesType
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.JavaScriptCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestEquals, TestSpec}
import _root_.io.kaitai.struct.translators.JavaScriptTranslator

class JavaScriptSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val className = JavaScriptCompiler.type2class(spec.id)
  val translator = new JavaScriptTranslator(provider, importList)

  importList.add("assert")

  override def fileName(name: String): String = s"test_${spec.id}.js"

  override def header(): Unit = {}

  override def runParse(): Unit = {
    importList.add("testHelper")

    out.puts(s"testHelper('$className', 'src/${spec.data}', function(r, $className) {")
    out.inc
    spec.extraImports.foreach { (entry) =>
      val entryClass = JavaScriptCompiler.type2class(entry)
      out.puts(s"var $entryClass = require('$entryClass').$entryClass;")
    }
    if (spec.debug) {
      out.puts("r._read();")
    }
  }

  override def runParseExpectError(exception: KSError): Unit = {
    importList.add("testHelperThrows")
    importList.add("KaitaiStream")

    out.puts(s"testHelperThrows('$className', 'src/${spec.data}', ${JavaScriptCompiler.ksErrorName(exception)});")
    if (spec.debug) {
      out.puts("r._read();")
    }
  }

  override def footer(): Unit = if (spec.exception.isEmpty) {
    out.dec
    out.puts("});")
  }

  override def simpleEquality(check: TestEquals): Unit = {
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

  override def floatEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"assert(Math.abs($actStr - $expStr) < $FLOAT_DELTA);")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"assert.strictEqual($actStr, undefined);")
  }

  override def trueArrayEquality(check: TestEquals, elType: DataType, elts: Seq[Ast.expr]): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"assert.deepStrictEqual($actStr, $expStr);")
  }

  override def testException(actual: Ast.expr, exception: KSError): Unit = {
    out.puts("assert.throws(")
    out.inc
    out.puts("function() {")
    out.inc
    out.puts(translateAct(actual) + ";")
    out.dec
    out.puts("},")
    out.puts(s"{name: '${JavaScriptCompiler.ksErrorName(exception)}'}")
    out.dec
    out.puts(")")
  }

  override def indentStr: String = "  "

  override def results: String = {
    "// " + AUTOGEN_COMMENT + "\n\n" +
      importList.toList.map((x) => {
        val fileName = x match {
          case "KaitaiStream" => "kaitai-struct/KaitaiStream"
          case other => other
        }
        s"var $x = require('$fileName');"
      }).mkString("", "\n", "\n") + "\n" +
      out.result
  }

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace(s"this.${Main.INIT_OBJ_NAME}", "r")
}
