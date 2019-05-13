package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import io.kaitai.struct.datatype.DataType
import io.kaitai.struct.exprlang.Ast
import io.kaitai.struct.{ClassTypeProvider, RuntimeConfig}
import io.kaitai.struct.languages.RustCompiler
import io.kaitai.struct.translators.RustTranslator

class RustSG(spec: TestSpec, provider: ClassTypeProvider)
    extends BaseGenerator(spec) {
  val className: String = RustCompiler.type2class(spec.id)
  val translator = new RustTranslator(provider, RuntimeConfig())

  override def fileName(name: String): String = s"tests/test_$name.rs"

  override def header(): Unit = {
    out.puts("use kaitai::{BytesReader, KStruct, KStructUnit};")
    out.puts(s"use kaitai_test::${spec.id}::$className;")
    out.puts(s"use std::fs;")
    out.puts

    out.puts("#[test]")
    out.puts(s"fn test_${spec.id}() {")
    out.inc

    out.puts("let data = fs::read(\"../../src/" + spec.data + "\").expect(\"Unable to read data.\");")
    out.puts(s"let reader = BytesReader::new(&data[..]);")
    out.puts(s"let r = $className::default().read(&reader, None, KStructUnit::parent_stack()).unwrap();")
  }

  override def footer(): Unit = {
    out.dec
    out.puts("}")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"// nullAsset($actStr);")
  }

  def translateAct(x: Ast.expr): String =
    translator.translate(x).replace("self." + Main.INIT_OBJ_NAME, "r")

  override def trueArrayAssert(check: TestAssert,
                               elType: DataType,
                               elts: Seq[Ast.expr]): Unit = {
    simpleAssert(check)
  }

  override def simpleAssert(check: TestAssert): Unit = {
    val actStr = translateAct(check.actual)
    val expStr = translator.translate(check.expected)
    out.puts(s"// assert_eq!($actStr, $expStr);")
  }

  override def indentStr: String = "    "

  override def results: String = "// " + AUTOGEN_COMMENT + "\n\n" + out.result
}
