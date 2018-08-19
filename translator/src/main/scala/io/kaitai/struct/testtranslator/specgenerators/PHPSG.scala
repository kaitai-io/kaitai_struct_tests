package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.{ClassTypeProvider, RuntimeConfig}
import io.kaitai.struct.datatype.DataType
import io.kaitai.struct.datatype.DataType.BytesType
import io.kaitai.struct.exprlang.Ast
import io.kaitai.struct.languages.PHPCompiler
import io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import io.kaitai.struct.translators.PHPTranslator

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
    out.puts(s"$$r = $className::fromFile(self::SRC_DIR_PATH . '/${spec.data}');")
    out.puts
  }

  override def footer(): Unit = {
    out.dec
    out.puts("}")
    out.dec
    out.puts("}")
  }

  override def simpleAssert(check: TestAssert): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"$$this->assertEquals($expStr, $actStr);")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"$$this->assertEquals(null, $actStr);")
  }

  override def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[Ast.expr]): Unit =
    simpleAssert(check)

  override def noAsserts(): Unit =
    out.puts("$this->markTestAsNotRisky();")

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
