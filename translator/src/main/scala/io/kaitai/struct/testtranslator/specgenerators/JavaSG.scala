package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.{ClassTypeProvider, RuntimeConfig}
import io.kaitai.struct.datatype.DataType._
import io.kaitai.struct.datatype.{DataType, KSError, EndOfStreamError}
import io.kaitai.struct.exprlang.Ast
import io.kaitai.struct.languages.JavaCompiler
import io.kaitai.struct.testtranslator.{Main, TestAssert, TestEquals, TestSpec, ExpectedException}
import io.kaitai.struct.translators.JavaTranslator

class JavaSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val config = RuntimeConfig()
  val className = JavaCompiler.type2class(spec.id)
  val translator = new JavaTranslator(provider, importList, config)
  val compiler = new JavaCompiler(provider, config)

  importList.add(s"io.kaitai.struct.testformats.$className")
  spec.extraImports.foreach(entry =>
    importList.add(s"io.kaitai.struct.testformats.${JavaCompiler.type2class(entry)}")
  )

  importList.add("org.testng.annotations.Test")
  importList.add("static org.testng.Assert.*")

  override def fileName(name: String): String = s"src/io/kaitai/struct/spec/Test$className.java"

  override def header(): Unit = {
    out.puts(s"public class Test$className extends CommonSpec {")
    out.inc
  }

  override def runParse(): Unit = {
    out.puts("@Test")
    runParseCommon()
  }

  override def runParseExpectError(expException: ExpectedException): Unit = {
    val exception = expException.exception
    if (exception == EndOfStreamError) {
      out.puts("@Test")
      out.puts(s"public void test$className() throws Exception {")
      out.inc

      out.puts("assertThrowsEofError(new ThrowingRunnable() {")
      out.inc

      out.puts("@Override")
      out.puts("public void run() throws Throwable {")
      out.inc
      out.puts(s"$className r = $className.fromFile(SRC_DIR + " + "\"" + spec.data + "\");")
      out.dec
      out.puts("}")

      out.dec
      out.puts("});")
    } else {
      importList.add("io.kaitai.struct.KaitaiStream")
      out.puts(s"@Test(expectedExceptions = ${compiler.ksErrorName(exception)}.class)")
      runParseCommon()
    }
  }

  def runParseCommon(): Unit = {
    out.puts(s"public void test$className() throws Exception {")
    out.inc
    out.puts(s"$className r = $className.fromFile(SRC_DIR + " + "\"" + spec.data + "\");")
  }

  override def footer(): Unit = {
    out.dec
    out.puts("}")
    out.dec
    out.puts("}")
  }

  override def simpleEquality(check: TestEquals): Unit = {
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

  override def floatEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"assertEquals($actStr, $expStr, $FLOAT_DELTA);")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"assertNull($actStr);")
  }

  override def trueArrayEquality(check: TestEquals, elType: DataType, elts: Seq[Ast.expr]): Unit = {
    simpleEquality(check) // FIXME
  }

  override def testException(actual: Ast.expr, exception: KSError): Unit = {
    importList.add("io.kaitai.struct.KaitaiStream")
    out.puts(s"assertThrows(${compiler.ksErrorName(exception)}.class, () -> ${translateAct(actual)});")
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
