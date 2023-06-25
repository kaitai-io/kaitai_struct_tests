package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.datatype.{DataType, KSError}
import io.kaitai.struct.exprlang.Ast
import io.kaitai.struct.languages.CCompiler
import io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import io.kaitai.struct.datatype.DataType._
import io.kaitai.struct.translators.CTranslator
import io.kaitai.struct.{ClassTypeProvider, RuntimeConfig}
import io.kaitai.struct.languages.components.CppImportList

class CSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val compiler = new CCompiler(provider, RuntimeConfig())
  val className = spec.id.toLowerCase()
  val cppImportList = new CppImportList
  val translator = new CTranslator(provider, cppImportList, false)

  override def fileName(name: String): String = s"test_$name.cpp"

  cppImportList.addLocal("operators.h")
  cppImportList.addSystem("boost/test/unit_test.hpp")
  spec.extraImports.foreach(entry => cppImportList.addLocal(s"$entry.h"))
  cppImportList.addSystem("stdio.h")

  override def header() = {
    out.puts("static void log(const char* text) {")
    out.puts("    printf(text);")
    out.puts("}")
    out.puts
    out.puts(s"BOOST_AUTO_TEST_CASE(test_${spec.id}) {")
    out.inc
  }

  override def runParse(): Unit = {
    runParseCommon1()
    out.puts(s"BOOST_CHECK_EQUAL(error, 0);")
    out.puts("if (error != 0) return;")
  }

  override def runParseExpectError(exception: KSError): Unit = {
    runParseCommon1()
    out.puts(s"BOOST_CHECK_EQUAL(error == 0, 0);")
  }

  def runParseCommon1(): Unit = {
    out.puts(s"ksx_$className* data;")
    out.puts("ks_stream* stream;")
    out.puts("ks_config* config;")
    out.puts("ks_error error;")
    out.puts("config = ks_config_create(log);")
    out.puts("FILE* file = fopen(\"src/" + spec.data + "\", \"r\");")
    out.puts("BOOST_CHECK_EQUAL(file != 0, 1);")
    out.puts("if (!file) return;")
    out.puts("stream = ks_stream_create_from_file(file, config);")
    out.puts(s"data = ksx_read_${className}_from_stream(stream, &error);")
    out.puts(s"(void)data;")
  }

  override def footer() = {
    out.puts
    out.puts("ks_config_destroy(config);")
    out.dec
    out.puts("}")
  }

  def simpleAssert(check: TestAssert): Unit = {
    val ptr = translator.detectType(check.expected) match {
      case t: StrType => "*"
      case t: BytesType => "*"
      case _ => ""
    }
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"BOOST_CHECK_EQUAL($ptr$actStr, $ptr$expStr);")
  }

  override def floatAssert(check: TestAssert): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"BOOST_CHECK_CLOSE($actStr, $expStr, 1e-4);")
  }

  def nullAssert(actual: Ast.expr): Unit = {
    val nullCheckStr = actual match {
      case Ast.expr.Attribute(x, Ast.identifier(attrName)) =>
       fixup("!" + translator.anyField(x, s"_is_valid_$attrName"))
    }
    out.puts(s"BOOST_CHECK($nullCheckStr);")
  }

  def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[Ast.expr]): Unit = {
    val elTypeName = CCompiler.kaitaiType2NativeType(elType)
    val eltsStr = elts.map(translator.translate).mkString(", ")
    val actStr = translateAct(check.actual)
    elType match {
      case t: StrType =>
        out.puts(s"COMPARE_ARRAY_POINTER(ks_string, $actStr, $eltsStr);")
      case _ =>
        out.puts(s"COMPARE_ARRAY($elTypeName, $actStr, $eltsStr);")
    }
  }

  override def indentStr: String = "    "

  override def results: String = {
    "// " + AUTOGEN_COMMENT + "\n\n" +
      "#define KS_USE_ZLIB\n" +
      "#define KS_DEPEND_ON_INTERNALS\n" +
      "#define KS_USE_ICONV\n" +
      "extern \"C\" {\n" +
      "#include \"" + spec.id + ".h\"\n" +
      "}\n" +
      cppImportList.result + "\n" +
      out.result
  }

  def fixup(s: String) : String = s.replace("->" + Main.INIT_OBJ_NAME, "")

  def translateAct(x: Ast.expr) = {
    fixup(translator.translate(x))
  }
}
