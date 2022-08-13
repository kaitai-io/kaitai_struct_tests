package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.{ClassTypeProvider, RuntimeConfig}
import io.kaitai.struct.datatype.DataType._
import io.kaitai.struct.datatype.{DataType, KSError}
import io.kaitai.struct.exprlang.Ast
import io.kaitai.struct.languages.JavaCompiler
import io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import io.kaitai.struct.translators.JavaTranslator

class JavaWriteSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val config = RuntimeConfig()
  val className = JavaCompiler.type2class(spec.id)
  val translator = new JavaTranslator(provider, importList)
  val compiler = new JavaCompiler(provider, config)

  importList.add("io.kaitai.struct.KaitaiStruct.ReadWrite")
  importList.add(s"io.kaitai.struct.testwrite.$className")

  importList.add("org.testng.annotations.Test")

  override def fileName(name: String): String = s"src/io/kaitai/struct/specwrite/Test$className.java"

  override def header(): Unit = {
    out.puts(s"public class Test$className extends CommonSpec {")
    out.inc
  }

  override def runParse(): Unit = {
    out.puts("@Override")
    out.puts("protected Class<? extends ReadWrite> getStructClass() {")
    out.inc
    out.puts(s"return $className.class;")
    out.dec
    out.puts("}")
    out.puts
    out.puts("@Override")
    out.puts("protected String getSrcFilename() {")
    out.inc
    out.puts("return \"" + spec.data + "\";")
    out.dec
    out.puts("}")
  }

  override def runParseExpectError(exception: KSError): Unit = {
    out.puts("@Override")
    out.puts("protected Class<? extends ReadWrite> getStructClass() {")
    out.inc
    out.puts("throw new UnsupportedOperationException();")
    out.dec
    out.puts("}")
    out.puts
    out.puts("@Override")
    out.puts("protected String getSrcFilename() {")
    out.inc
    out.puts("throw new UnsupportedOperationException();")
    out.dec
    out.puts("}")
  }

  override def footer(): Unit = {
    out.dec
    out.puts("}")
  }

  override def runAsserts(): Unit = {}

  def simpleAssert(check: TestAssert): Unit = ???

  def nullAssert(actual: Ast.expr): Unit = ???

  def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[Ast.expr]): Unit = ???

  override def indentStr: String = "    "

  override def results: String = {
    "// " + AUTOGEN_COMMENT + "\n\n" +
      "package io.kaitai.struct.specwrite;\n\n" +
      importList.toList.map((x) => s"import $x;").mkString("", "\n", "\n\n") +
      out.result
  }
}
