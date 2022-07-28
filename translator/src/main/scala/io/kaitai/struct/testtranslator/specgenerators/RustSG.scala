package io.kaitai.struct.testtranslator.specgenerators

import _root_.io.kaitai.struct.{ClassTypeProvider, RuntimeConfig}
import _root_.io.kaitai.struct.datatype.{DataType, KSError}
import _root_.io.kaitai.struct.datatype.DataType._
import _root_.io.kaitai.struct.exprlang.Ast
import _root_.io.kaitai.struct.languages.RustCompiler
import _root_.io.kaitai.struct.testtranslator.{Main, TestAssert, TestSpec}
import _root_.io.kaitai.struct.translators.RustTranslator

class RustSG(spec: TestSpec, provider: ClassTypeProvider) extends BaseGenerator(spec) {
  val className = RustCompiler.type2class(spec.id)
  val translator = new RustTranslator(provider, RuntimeConfig())
  var do_panic = true;

  override def fileName(name: String): String = s"test_$name.rs"

  override def header(): Unit = {
    val code =
      s"""|use std::fs;
          |
          |extern crate kaitai;
          |use self::kaitai::*;
          |mod formats;
          |use formats::${spec.id}::*;
          |
          |#[test]
          |fn test_${spec.id}() {
          |    let bytes = fs::read("../../src/${spec.data}").unwrap();
          |    let reader = BytesReader::new(&bytes);
          |    let mut r = $className::default();
          |
          |    if let Err(err) = r.read(&reader, None, KStructUnit::parent_stack()) {""".stripMargin
    out.puts(code)
    out.inc
  }

  override def runParseExpectError(exception: KSError): Unit = {
    val code =
      s"""    println!("expected err: {:?}, exception: ${exception}", err);
      |    } else {
      |        panic!("no expected exception: ${exception}");
      |    }""".stripMargin
    out.puts(code)
    do_panic = false
    //    out.puts("err = r.Read(s, &r, &r)")
//    out.puts("switch v := err.(type) {")
//    out.puts(s"case ${errorName}:")
//    out.inc
//    out.puts("break")
//    out.dec
//    out.puts("default:")
//    out.inc
//    out.puts("t.Fatalf(\"expected " + errorName + ", got %T\", v)")
//    out.dec
//    out.puts("}")
  }

  def finish_panic(): Unit = {
    if (do_panic) {
      out.inc
      out.puts("panic!(\"{:?}\", err);")
      out.dec
      out.puts("}")
      do_panic = false
    }
  }
  override def footer(): Unit = {
    out.dec
    out.puts("}")
  }

  override def simpleAssert(check: TestAssert): Unit = {
    val actStr = translateAct(check.actual)
    var expStr = translator.translate(check.expected)
    if (expStr.toLowerCase.startsWith("enum")) {
      expStr = s"Some($expStr)"
    }
    finish_panic()
    out.puts(s"assert_eq!($actStr, $expStr);")
  }

  override def nullAssert(actual: Ast.expr): Unit = {
    val actStr = translateAct(actual)
    finish_panic()
    out.puts(s"assert_eq!($actStr, 0);")
    // TODO: Figure out what's meant to happen here
  }

  override def trueArrayAssert(check: TestAssert, elType: DataType, elts: Seq[Ast.expr]): Unit = {
    out.puts("trueArrayAssert $check, $elType, $elts")
    simpleAssert(check) // FIXME
  }

  override def indentStr: String = "    "

  override def results: String = {
    "// " + AUTOGEN_COMMENT + "\n\n" +
      out.result
  }

  def translateAct(x: Ast.expr) =
    translator.translate(x).replace("self." + Main.INIT_OBJ_NAME + "()", "r")
}
