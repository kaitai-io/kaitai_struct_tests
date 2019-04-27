package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.ClassTypeProvider
import _root_.io.kaitai.struct.datatype.DataType
import _root_.io.kaitai.struct.datatype.DataType._
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.JavaCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import _root_.io.kaitai.struct.translators.JavaTranslator

class JavaSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val className = JavaCompiler.type2class(spec.id)
  val translator = new JavaTranslator(provider, importList)

  importList.add(s"io.kaitai.struct.testformats.$className")
  importList.add("org.testng.annotations.Test")
  importList.add("static org.testng.Assert.*")

  override def fileName(name: String): String = s"src/io/kaitai/struct/spec/Test$className.java"

  override def header(): Unit = {
    out.puts(s"public class Test$className extends CommonSpec {")
    out.inc
    out.puts("@Test")
    out.puts(s"public void test$className() throws Exception {")
    out.inc
    out.puts(s"$className r = $className.fromFile(SRC_DIR + " + "\"" + spec.data + "\");")
    out.puts
  }

  override def footer(): Unit = {
    out.dec
    out.puts("}")
    out.dec
    out.puts("}")
  }

  override def simpleAssert(check: TestAssert): Unit = {
    val actType = translator.detectType(check.actual)
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    actType match {
      case _: IntType | _: BooleanType =>
        out.puts(s"assertIntEquals($actStr, $expStr);")
      case _ =>
        out.puts(s"assertEquals($actStr, $expStr);")
    }
  }

  override def floatAssert(check: TestAssert): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"assertEquals($actStr, $expStr, $FLOAT_DELTA);")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"assertNull($actStr);")
  }

  override def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[Ast.expr]): Unit = {
    simpleAssert(check) // FIXME
  }

  override def indentStr: String = "    "

  override def results: String = {
    "// " + AUTOGEN_COMMENT + "\n\n" +
      "package io.kaitai.struct.spec;\n\n" +
      importList.toList.map((x) => s"import $x;").mkString("", "\n", "\n") +
      out.result
  }

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace(Main.INIT_OBJ_NAME + "()", "r")
}
