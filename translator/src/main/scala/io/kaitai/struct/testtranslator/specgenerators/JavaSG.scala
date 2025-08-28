package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.{ClassTypeProvider, RuntimeConfig}
import io.kaitai.struct.datatype.DataType._
import io.kaitai.struct.datatype.{DataType, KSError}
import io.kaitai.struct.exprlang.Ast
import io.kaitai.struct.languages.JavaCompiler
import io.kaitai.struct.testtranslator.{Main, TestAssert, TestEquals, TestException, TestSpec, ExpectedException}
import io.kaitai.struct.translators.JavaTranslator

class JavaSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val config = RuntimeConfig()
  val className = JavaCompiler.type2class(spec.id)
  val translator = new JavaTranslator(provider, importList)
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
    out.puts
  }

  override def runParse(): Unit = {
    out.puts("@Test")
    runParseCommon()
  }

  override def runParseExpectError(expException: ExpectedException): Unit = {
    val exception = expException.exception
    importList.add("io.kaitai.struct.KaitaiStream")
    out.puts(s"@Test(expectedExceptions = ${compiler.ksErrorName(exception)}.class)")
    runParseCommon()
  }

  def runParseCommon(): Unit = {
    out.puts(s"public void test$className() throws Exception {")
    out.inc
    /** If [[testException]] will be called, we have to add the `final` keyword
     * when declaring `r` for Java 7 compatibility, otherwise "error: local
     * variable r is accessed from within inner class; needs to be declared
     * final" will occur. This is not needed since Java 8 - I suppose this is
     * because Java 8 introduces the concept of "effectively final", so it's no
     * longer required to specify `final` explicitly, as it is inferred
     * automatically. */
    val needsFinal = spec.asserts.exists(assert => assert.isInstanceOf[TestException])
    val finalKeyword = if (needsFinal) "final " else ""
    out.puts(s"${finalKeyword}$className r = $className.fromFile(SRC_DIR + " + "\"" + spec.data + "\");")
  }

  override def footer(): Unit = {
    out.dec
    out.puts("}")
    out.dec
    out.puts("}")
  }

  override def simpleEquality(check: TestEquals): Unit = {
    val actType = translator.detectType(check.actual)
    val expType = translator.detectType(check.expected)

    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)

    (actType, expType) match {
      case (_: IntType | _: BooleanType, _) =>
        out.puts(s"assertIntEquals($actStr, $expStr);")
      case (et: EnumType, _: IntType) => {
        // When we expect unknown value of enumeration, in KST we uses just it integer value
        // That expression would be translated to number. Wh should wrap it to enum
        val enumSpec = et.enumSpec.get
        val expEnum = translator.doEnumById(enumSpec, expStr)
        val enumName = translator.enumClass(enumSpec.name)

        out.puts(s"assertEquals($actStr, $expEnum);")
        out.puts(s"assertTrue($expEnum instanceof $enumName.Unknown);")
      }
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

    out.puts(s"assertThrows(${compiler.ksErrorName(exception)}.class, new ThrowingRunnable() {")
    out.inc

    out.puts("@Override")
    out.puts("public void run() throws Throwable {")
    out.inc
    out.puts(s"${translateAct(actual)};")
    out.dec
    out.puts("}")

    out.dec
    out.puts("});")
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
