package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.{ClassTypeProvider, RuntimeConfig}
import _root_.io.kaitai.struct.datatype.DataType
import _root_.io.kaitai.struct.datatype.DataType._
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.RustCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import _root_.io.kaitai.struct.translators.RustTranslator

class RustSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val className = RustCompiler.type2class(spec.id)
  val translator = new RustTranslator(provider, RuntimeConfig())

  override def fileName(name: String): String = s"test_$name.rs"

  override def header(): Unit = {
    val code =
      s"""
        |#[cfg(test)]
        |mod tests {
        |    extern crate kaitai_struct;
        |    use std::fs;
        |
        |    use kaitai_struct::*;
        |    use crate::rust::${spec.id}::*;
        |
        |    #[test]
        |    fn test_${spec.id}() {
        |        let bytes = fs::read("src/${spec.data}").unwrap();
        |        let mut reader = BytesReader::new(&bytes);
        |        let mut r = $className::default();
        |
        |        r.read(&mut reader, None, None).unwrap();
        |""".stripMargin

    out.puts(code)
    out.inc
    out.inc
  }

  override def footer(): Unit = {
    out.dec
    out.puts("}")
    out.dec
    out.puts("}")
  }

  override def simpleAssert(check: TestAssert): Unit = {
    val actStr = translateAct(check.actual)
    var expStr = translator.translate(check.expected)
    if (expStr.toLowerCase.startsWith("enum")) {
      expStr = s"Some($expStr)"
    }
    out.puts(s"assert_eq!($actStr, $expStr);")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    out.puts(s"assertNull($actStr);")
    // TODO: Figure out what's meant to happen here
  }

  override def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[Ast.expr]): Unit = {
    simpleAssert(check) // FIXME
  }

  override def indentStr: String = "    "

  override def results: String = {
    "// " + AUTOGEN_COMMENT + "\n\n" +
      out.result
  }

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace("self." + Main.INIT_OBJ_NAME, "r")
}
