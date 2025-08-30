package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.{ClassTypeProvider, RuntimeConfig}
import io.kaitai.struct.datatype.DataType._
import io.kaitai.struct.datatype.{DataType, KSError}
import io.kaitai.struct.exprlang.Ast
import io.kaitai.struct.languages.JavaCompiler
import io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import io.kaitai.struct.translators.JavaTranslator

class JavaWriteSG(spec: TestSpec, provider: ClassTypeProvider) extends SpecGenerator {
  val config = RuntimeConfig()
  val className = JavaCompiler.type2class(spec.id)
  val translator = new JavaTranslator(provider, importList, config)
  val compiler = new JavaCompiler(provider, config)

  importList.add("io.kaitai.struct.KaitaiStruct.ReadWrite")
  importList.add(s"io.kaitai.struct.testwrite.$className")

  importList.add("org.testng.annotations.Test")

  override def fileName(name: String): String = s"src/io/kaitai/struct/specwrite/Test$className.java"

  override def run(): Unit = {
    out.puts(s"public class Test$className extends CommonSpec {")
    out.inc

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

    spec.exception match {
      case None =>
      case Some(_) =>
        importList.add("org.testng.SkipException")

        out.puts
        out.puts("@Override")
        out.puts("@Test")
        out.puts("protected void testReadWriteRoundtrip() throws Exception {")
        out.inc
        out.puts("""throw new SkipException("cannot use roundtrip because parsing is expected to fail");""")
        out.dec
        out.puts("}")
    }

    out.dec
    out.puts("}")
  }

  override def indentStr: String = "    "

  override def results: String = {
    "// " + AUTOGEN_COMMENT + "\n\n" +
      "package io.kaitai.struct.specwrite;\n\n" +
      importList.toList.map((x) => s"import $x;").mkString("", "\n", "\n\n") +
      out.result
  }
}
