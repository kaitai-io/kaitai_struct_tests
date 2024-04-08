package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.{ClassTypeProvider, RuntimeConfig}
import _root_.io.kaitai.struct.datatype.DataType
import _root_.io.kaitai.struct.datatype.DataType._
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.RustCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestEquals, TestSpec}
import _root_.io.kaitai.struct.translators.RustTranslator

class RustSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val className = RustCompiler.type2class(spec.id)
  val translator = new RustTranslator(provider, RuntimeConfig())

  override def fileName(name: String): String = s"tests/$name.rs"

  override def header(): Unit = {
    out.puts("use kaitai_struct::KaitaiStruct;")
    out.puts(s"use kaitai_test_suite::${spec.id}::$className;")
    out.puts

    out.puts("#[test]")
    out.puts(s"fn test_${spec.id}() {")
    out.inc

    out.puts(s"let r = $className::from_file(\"../../src/${spec.data}\").expect(\"file for parsing is not found\");")
  }

  override def footer(): Unit = {
    out.dec
    out.puts("}")
  }

  override def simpleEquality(check: TestEquals): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"assert_eq!($actStr, $expStr);")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"assertNull($actStr);")
    // TODO: Figure out what's meant to happen here
  }

  override def trueArrayEquality(check: TestEquals, elType: DataType, elts: Seq[Ast.expr]): Unit = {
    simpleEquality(check) // FIXME
  }

  override def indentStr: String = "    "

  override def results: String = {
    "// " + AUTOGEN_COMMENT + "\n\n" +
      out.result
  }

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace("self." + Main.INIT_OBJ_NAME, "r")
}
