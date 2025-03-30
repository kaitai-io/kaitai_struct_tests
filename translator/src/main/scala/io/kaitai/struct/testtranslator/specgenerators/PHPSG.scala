package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.{ClassTypeProvider, RuntimeConfig}
import _root_.io.kaitai.struct.datatype.{DataType, KSError}
import _root_.io.kaitai.struct.datatype.DataType.BytesType
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.PHPCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestEquals, TestSpec, ExpectedException}
import _root_.io.kaitai.struct.translators.PHPTranslator

class PHPSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val namespace = "Kaitai\\Struct\\Tests"
  val className = PHPCompiler.type2class(spec.id)
  val translator = new PHPTranslator(provider, RuntimeConfig(phpNamespace = namespace))

  override def fileName(name: String): String = s"${className}Test.php"

  override def header(): Unit = {
    out.puts(s"namespace $namespace;")
    out.puts
    out.puts(s"class ${className}Test extends TestCase {")
    out.inc
    out.puts(s"public function test$className() {")
    out.inc
  }

  override def runParse(): Unit = {
    out.puts(s"$$r = $className::fromFile(self::SRC_DIR_PATH . '/${spec.data}');")
  }

  override def runParseExpectError(expException: ExpectedException): Unit = {
    val exception = expException.exception
    out.puts(s"$$this->expectException(${PHPCompiler.ksErrorName(exception)}::class);")
    runParse()
  }

  override def footer(): Unit = {
    out.dec
    out.puts("}")
    out.dec
    out.puts("}")
  }

  override def simpleEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"$$this->assertSame($expStr, $actStr);")
  }

  override def floatEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"$$this->assertEquals($actStr, $expStr, '', $FLOAT_DELTA);")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"$$this->assertNull($actStr);")
  }

  override def trueArrayEquality(check: TestEquals, elType: DataType, elts: Seq[Ast.expr]): Unit =
    simpleEquality(check)

  override def indentStr: String = "    "

  override def results: String = {
    "<?php\n" +
    "// " + AUTOGEN_COMMENT + "\n\n" +
      // importList.toList.mkString("", "\n", "\n") + "\n" +
      out.result
  }

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace(s"$$this->${Main.INIT_OBJ_NAME}()", "$r")
}
