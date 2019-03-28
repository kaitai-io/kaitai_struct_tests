package io.kaitai.struct.testtranslator.specgenerators

import io.kaitai.struct.datatype.DataType
import io.kaitai.struct.exprlang.Ast
import io.kaitai.struct.languages.CppCompiler
import io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import io.kaitai.struct.translators.CppTranslator
import io.kaitai.struct.{ClassTypeProvider, CppRuntimeConfig, RuntimeConfig}

class CppStlSG(spec: TestSpec, provider: ClassTypeProvider, cppConfig: CppRuntimeConfig) extends BaseGenerator(spec) {
  val config = RuntimeConfig(cppConfig = cppConfig)
  val compiler = new CppCompiler(provider, config)
  val className = CppCompiler.types2class(List(spec.id))
  val translator = new CppTranslator(provider, importList, config)

  override def fileName(name: String): String = s"test_$name.cpp"

  importList.add("<boost/test/unit_test.hpp>")
  importList.add(s"<${spec.id}.h>")
  importList.add("<iostream>")
  importList.add("<fstream>")
  importList.add("<vector>")

  override def header() = {
    out.puts
    out.puts(s"BOOST_AUTO_TEST_CASE(test_${spec.id}) {")
    out.inc
    out.puts("std::ifstream ifs(\"src/" + spec.data + "\", std::ifstream::binary);")
    out.puts("kaitai::kstream ks(&ifs);")
    out.puts(s"$className* r = new $className(&ks);")
    out.puts
  }

  override def footer() = {
    out.puts
    out.puts("delete r;")
    out.dec
    out.puts("}")
  }

  def simpleAssert(check: TestAssert): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"BOOST_CHECK_EQUAL($actStr, $expStr);")
  }

  def nullAssert(actual: Ast.expr): Unit = {
    val nullCheckStr = actual match {
      case Ast.expr.Attribute(x, Ast.identifier(attrName)) =>
        translateAct(x) + s"->_is_null_$attrName()"
    }
    out.puts(s"BOOST_CHECK($nullCheckStr);")
  }

  def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[Ast.expr]): Unit = {
    importList.add("\"helpers.h\"")
    val elTypeName = compiler.kaitaiType2NativeType(elType)
    val eltsStr = elts.map((x) => translator.translate(x)).mkString(", ")
    val actStr = translateAct(check.actual)
    out.puts(s"COMPARE_ARRAY($elTypeName, $actStr, $eltsStr);")
  }

  override def indentStr: String = "    "

  override def results: String = {
    "// " + AUTOGEN_COMMENT + "\n\n" +
      importList.toList.map((x) => s"#include $x").mkString("", "\n", "\n") +
      out.result
  }

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace(Main.INIT_OBJ_NAME + "()", "r")
}
