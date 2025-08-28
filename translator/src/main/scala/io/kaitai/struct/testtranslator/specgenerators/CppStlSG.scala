package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.datatype.{DataType, KSError}
import io.kaitai.struct.exprlang.Ast
import io.kaitai.struct.languages.CppCompiler
import io.kaitai.struct.testtranslator.{Main, TestAssert, TestEquals, TestSpec, ExpectedException}
import io.kaitai.struct.translators.CppTranslator
import io.kaitai.struct.{ClassTypeProvider, CppRuntimeConfig, RuntimeConfig}
import io.kaitai.struct.languages.components.CppImportList

class CppStlSG(spec: TestSpec, provider: ClassTypeProvider, cppConfig: CppRuntimeConfig) extends BaseGenerator(spec) {
  val config = RuntimeConfig(cppConfig = cppConfig)
  val compiler = new CppCompiler(provider, config)
  val className = CppCompiler.types2class(List(spec.id))
  val cppImportList = new CppImportList
  val hdrImportList = new CppImportList
  val translator = new CppTranslator(provider, cppImportList, hdrImportList, config)

  override def fileName(name: String): String = s"test_$name.cpp"

  cppImportList.addSystem("boost/test/unit_test.hpp")
  cppImportList.addLocal(s"${spec.id}.h")
  spec.extraImports.foreach(entry => cppImportList.addLocal(s"$entry.h"))
  cppImportList.addSystem("iostream")
  cppImportList.addSystem("fstream")
  cppImportList.addSystem("vector")

  override def header() = {
    out.puts(s"BOOST_AUTO_TEST_CASE(test_${spec.id}) {")
    out.inc
  }

  override def runParse(): Unit = {
    runParseCommon1()
    out.puts(s"$className* r = new $className(&ks);")
  }

  override def runParseExpectError(expException: ExpectedException): Unit = {
    val exception = expException.exception
    cppImportList.addKaitai("kaitai/exceptions.h")

    runParseCommon1()
    out.puts(s"$className* r = ${compiler.nullPtr};")
    out.puts("BOOST_CHECK_THROW(")
    out.inc
    out.puts(s"r = new $className(&ks),")
    out.puts(compiler.ksErrorName(exception))
    out.dec
    out.puts(");")
  }

  def runParseCommon1(): Unit = {
    out.puts("std::ifstream ifs(\"src/" + spec.data + "\", std::ifstream::binary);")
    out.puts("kaitai::kstream ks(&ifs);")
  }

  override def footer() = {
    out.puts
    out.puts("delete r;")
    out.dec
    out.puts("}")
  }

  override def simpleEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"BOOST_CHECK_EQUAL($actStr, $expStr);")
  }

  override def floatEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"BOOST_CHECK_CLOSE($actStr, $expStr, 1e-4);")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val nullCheckStr = actual match {
      case Ast.expr.Attribute(x, Ast.identifier(attrName)) =>
        translateAct(x) + s"->_is_null_$attrName()"
    }
    out.puts(s"BOOST_CHECK($nullCheckStr);")
  }

  override def trueArrayEquality(check: TestEquals, elType: DataType, elts: Seq[Ast.expr]): Unit = {
    cppImportList.addLocal("helpers.h")
    val elTypeName = compiler.kaitaiType2NativeType(elType)
    val eltsStr = elts.map((x) => translator.translate(x)).mkString(", ")
    val actStr = translateAct(check.actual)
    out.puts(s"COMPARE_ARRAY($elTypeName, $actStr, $eltsStr);")
  }

  override def testException(actual: Ast.expr, exception: KSError): Unit = {
    cppImportList.addKaitai("kaitai/exceptions.h")

    out.puts("BOOST_CHECK_THROW(")
    out.inc
    out.puts(translateAct(actual) + ",")
    out.puts(compiler.ksErrorName(exception))
    out.dec
    out.puts(");")
  }

  override def indentStr: String = "    "

  override def results: String = {
    "// " + AUTOGEN_COMMENT + "\n\n" +
      cppImportList.result + "\n" +
      out.result
  }

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace(Main.INIT_OBJ_NAME + "()", "r")
}
